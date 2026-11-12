use std::path::Path;

use anyhow::Context;
use object_store::local::LocalFileSystem;

pub fn local_store(root: impl AsRef<Path>) -> anyhow::Result<LocalFileSystem> {
    let root = root.as_ref();
    std::fs::create_dir_all(root)
        .with_context(|| format!("Cannot create object storage directory {}", root.display()))?;
    LocalFileSystem::new_with_prefix(root).context("Cannot initialize local object storage")
}

#[cfg(test)]
mod tests {
    use super::local_store;
    use object_store::{ObjectStore, path::Path};

    #[tokio::test]
    async fn objects_persist_when_store_is_reopened() {
        let root = std::env::temp_dir().join(format!("registry-storage-{}", uuid::Uuid::new_v4()));
        let path = Path::from("artifacts/example.json");
        let payload = b"{\"name\":\"example\"}".to_vec();
        {
            let store = local_store(&root).unwrap();
            store.put(&path, payload.clone().into()).await.unwrap();
        }

        let reopened = local_store(&root).unwrap();
        let actual = reopened.get(&path).await.unwrap().bytes().await.unwrap();
        assert_eq!(actual.as_ref(), payload.as_slice());
        reopened.delete(&path).await.unwrap();
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn rejects_a_file_as_the_storage_directory() {
        let root = std::env::temp_dir().join(format!("registry-storage-{}", uuid::Uuid::new_v4()));
        std::fs::write(&root, b"not a directory").unwrap();
        assert!(local_store(&root).is_err());
        std::fs::remove_file(root).unwrap();
    }
}
