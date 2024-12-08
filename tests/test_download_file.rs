use async_ssh2_tokio::{AuthMethod, Client, ServerCheckMethod};
use tokio::fs;

#[tokio::test]
async fn test_download_file() {
    // Подключение к SSH-серверу
    let client = Client::connect(
        "192.168.0.110:22",                  // Адрес и порт сервера
        "chappic",                // Имя пользователя
        AuthMethod::Password("chappic".to_string()), // Метод аутентификации
        ServerCheckMethod::NoCheck,     // Игнорируем проверку хоста
    )
    .await
    .expect("Failed to connect to SSH server");

    // Укажите путь к удаленному файлу и локальный путь для сохранения
    let remote_path = "/home/chappic/yup.gif";
    let local_path = "/tmp/test_file_gif.gif";

    // Скачиваем файл
    client
        .download_file(remote_path, local_path)
        .await
        .expect("Failed to download file");

    // Проверяем содержимое файла
    let content = fs::read_to_string(local_path)
        .await
        .expect("Failed to read downloaded file");
    assert_eq!(content, "Expected file content\\n");

    // Удаляем файл после проверки
    fs::remove_file(local_path)
        .await
        .expect("Failed to delete downloaded file");
}
