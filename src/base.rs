use tokio::fs::File;
use tokio::io::AsyncReadExt;
pub async fn file_openString(name_file:&str) ->String{
    let mut file = File::open(name_file).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    contents
}