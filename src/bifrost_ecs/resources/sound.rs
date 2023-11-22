use soloud::{ AudioExt, LoadExt, Soloud, Wav};

use super::AssetLoader;

#[derive(Debug)]
pub struct Sound {
    sl: Soloud,
    sink: Wav,
}

unsafe impl Send for Sound {}
unsafe impl Sync for Sound {}

impl AssetLoader for Sound {
    type Asset = Self;

    fn load(
        _scene: &crate::bifrost_ecs::core::scene::Scene,
        path: &str,
    ) -> Result<std::sync::Arc<Self::Asset>, super::asset_loader_errors::AssetLoaderError> {
        let sl = Soloud::default().unwrap();

        let mut sink = Wav::default();

        sink.load(&path).unwrap();

        let sound = Sound { sl, sink };
        Ok(std::sync::Arc::new(sound))
    }
}

impl Sound {
    pub fn play(&self) {
        self.sl.play(&self.sink);
    }
}
