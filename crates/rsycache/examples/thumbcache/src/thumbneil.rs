use image::{DynamicImage, ImageFormat, io::Reader as ImageReader};
use sha2::{Digest, Sha256};
use std::fs::{self, File, metadata};
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ThumbnailError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Unsupported image format")]
    UnsupportedFormat,
}

pub struct ThumbnailCache {
    cache_dir: PathBuf,
    default_width: u32,
    default_height: u32,
}

impl ThumbnailCache {
    pub fn new<P: AsRef<Path>>(
        cache_dir: P,
        width: u32,
        height: u32,
    ) -> Result<Self, ThumbnailError> {
        let cache_dir = cache_dir.as_ref().to_path_buf();

        // 创建缓存目录
        fs::create_dir_all(&cache_dir)?;

        Ok(Self {
            cache_dir,
            default_width: width,
            default_height: height,
        })
    }

    /// 获取缩略图路径，如果不存在则生成
    pub fn get_thumbnail<P: AsRef<Path>>(
        &self,
        image_path: P,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<PathBuf, ThumbnailError> {
        let width = width.unwrap_or(self.default_width);
        let height = height.unwrap_or(self.default_height);

        let thumbnail_path = self.generate_thumbnail_path(&image_path, width, height)?;

        // 检查缓存是否有效
        if self.is_cache_valid(&image_path, &thumbnail_path)? {
            return Ok(thumbnail_path);
        }

        // 生成并保存缩略图
        self.generate_and_save_thumbnail(&image_path, width, height, &thumbnail_path)?;

        Ok(thumbnail_path)
    }

    /// 生成缓存文件名（使用原文件路径哈希 + 尺寸）
    fn generate_thumbnail_path<P: AsRef<Path>>(
        &self,
        image_path: P,
        width: u32,
        height: u32,
    ) -> Result<PathBuf, ThumbnailError> {
        let path_str = image_path.as_ref().to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let filename = format!("{}_{}x{}.webp", hash, width, height);
        Ok(self.cache_dir.join(filename))
    }

    /// 检查缓存是否有效（存在且比原文件新）
    fn is_cache_valid<P: AsRef<Path>>(
        &self,
        image_path: P,
        thumbnail_path: &Path,
    ) -> Result<bool, ThumbnailError> {
        // 获取原文件修改时间
        let image_modified = metadata(image_path)?.modified()?;

        match metadata(thumbnail_path) {
            Ok(meta) => {
                let thumb_modified = meta.modified()?;
                Ok(thumb_modified >= image_modified)
            }
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// 生成并保存缩略图
    fn generate_and_save_thumbnail<P: AsRef<Path>>(
        &self,
        image_path: P,
        width: u32,
        height: u32,
        thumbnail_path: &Path,
    ) -> Result<(), ThumbnailError> {
        // 读取原始图像
        let img = ImageReader::open(&image_path)?
            .with_guessed_format()?
            .decode()?;

        // 调整尺寸（保持宽高比）
        let thumbnail = img.resize_to_fill(width, height, image::imageops::FilterType::Lanczos3);

        // 保存为 WebP 格式
        let mut output = File::create(thumbnail_path)?;
        thumbnail.write_to(&mut output, ImageFormat::WebP)?;

        // 同步修改时间
        let mtime = filetime::FileTime::from_system_time(metadata(image_path)?.modified()?);
        filetime::set_file_mtime(thumbnail_path, mtime)?;

        Ok(())
    }

    /// 清理过期缓存（示例方法）
    pub fn cleanup_expired(&self, max_age_days: u32) -> Result<(), ThumbnailError> {
        let now = SystemTime::now();
        let max_age = std::time::Duration::from_secs(max_age_days as u64);

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Ok(modified) = entry.metadata()?.modified() {
                if let Ok(age) = now.duration_since(modified) {
                    if age > max_age {
                        fs::remove_file(path)?;
                    }
                }
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;
    use image::GenericImageView;
    use tempfile::NamedTempFile;
    use tempfile::tempdir;

    #[test]
    fn test_thumbnail_generation() {
        // let temp_dir = tempdir().unwrap();
        let temp_dir = tempfile::Builder::new().keep(true).tempdir().unwrap();
        println!("Temporary directory path: {:?}", temp_dir.path());
        let cache = ThumbnailCache::new(temp_dir.path(), 100, 100).unwrap();

        // Create a temporary image file with a .jpg extension for testing
        let temp_image = NamedTempFile::new().unwrap();
        let temp_image_path = temp_dir
            .path()
            .join(temp_image.path().with_extension("jpg"));
        println!("Temporary image path: {:?}", temp_image_path);

        // Write a simple image to the temporary file
        let image = DynamicImage::new_rgb8(200, 200);
        image.save(&temp_image_path).unwrap();

        let thumb_path = cache.get_thumbnail(&temp_image_path, None, None).unwrap();
        let thumb_img = image::open(thumb_path).unwrap();
        println!("thumb dim: {:?}", thumb_img.dimensions());
        // cache.cleanup_expired(60).unwrap();
        // assert!(thumb_path.exists());
        // let metadata = fs::metadata(thumb_path).unwrap();
        // println!("Thumbnail file size: {:?}", metadata);
        // assert!(metadata.len() > 0);
    }
}
