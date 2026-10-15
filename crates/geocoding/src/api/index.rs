use domain::params::api_config::GEOCODING_API_URL;
use domain::params::map_config::MAP_ZOOM_LEVEL;
use domain::types::gis_engine_type::MapXYZ;

/// 住所を地理院ジオコーディングAPIで検索する。
pub fn geocode_address(
    text: String,
    on_complete: impl FnOnce(Result<MapXYZ, String>) + Send + 'static,
) {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(async move {
        on_complete(request_geocode(&text).await);
    });

    #[cfg(not(target_arch = "wasm32"))]
    std::thread::spawn(move || {
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| error.to_string())
            .and_then(|runtime| runtime.block_on(request_geocode(&text)));
        on_complete(result);
    });
}

/// 非同期で住所を地理院ジオコーディングAPIに問い合わせる。
/// # 引数
/// - `text`: 検索する住所の文字列。
///
/// # 戻り値
/// `MapXYZ`型の座標情報を含む`Result`。
async fn request_geocode(text: &str) -> Result<MapXYZ, String> {
    let response = reqwest::Client::new()
        .get(GEOCODING_API_URL)
        .query(&[("q", text)])
        .send()
        .await
        .map_err(|error| error.to_string())?
        .error_for_status()
        .map_err(|error| error.to_string())?;

    let results: serde_json::Value = response.json().await.map_err(|error| error.to_string())?;

    let (longitude, latitude) = validate_geocoding_response(&results)?;

    Ok(MapXYZ {
        longitude,
        latitude,
        zoom: MAP_ZOOM_LEVEL,
    })
}

/// APIレスポンスを検証し、座標を取得する。
/// # 引数
/// - `results`: 地理院ジオコーディングAPIからのJSONレスポンス。
///
/// # 戻り値
/// `(経度, 緯度)`のタプルを含む`Result`。
fn validate_geocoding_response(results: &serde_json::Value) -> Result<(f64, f64), String> {
    results
        .get(0)
        .and_then(|result| result.get("geometry"))
        .and_then(|geometry| geometry.get("coordinates"))
        .and_then(serde_json::Value::as_array)
        .and_then(|coordinates| {
            Some((
                coordinates.first()?.as_f64()?,
                coordinates.get(1)?.as_f64()?,
            ))
        })
        .ok_or_else(|| "APIレスポンスが不正です。".to_owned())
}
