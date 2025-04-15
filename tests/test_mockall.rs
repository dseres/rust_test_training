use mockall::*;
use mockall::predicate::*;

#[automock]
trait Downloader {
  fn download(&self, url: &str) -> String;
}

fn download(my_struct: &dyn Downloader) -> String {
    my_struct.download( "https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt")
}

#[test]
fn test_download() {
  let mut mock = MockDownloader::new();
 
  let image_txt = std::fs::read_to_string("examples/image.txt").unwrap();
  mock.expect_download()
      .with(predicate::eq("https://raw.githubusercontent.com/dseres/rust_test_training/refs/heads/master/examples/image.txt"))
      .return_const(image_txt);

  
  assert_eq!(50, download(&mock).lines().count(), "Downloaded file should have 50 lines");
}