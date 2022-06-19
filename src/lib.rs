pub(crate) mod gecko {
    use serde::de::DeserializeOwned;

    const ORIGIN: &str = "https://api.coingecko.com/api/v3";

    pub fn vec_str_2_comma_str(vector: Vec<&str>) -> String {
        let vec_len = vector.len();
        let mut i = 0;
        let mut vector_str = String::new();

        for item in vector {
            i += 1;
            vector_str.push_str(item);
            if i < vec_len {
                vector_str.push_str(",");
            }
        }
        vector_str
    }

    pub fn get_request<T: DeserializeOwned>(
        endpoint: &str,
        params: &str,
    ) -> (reqwest::StatusCode, T) {
        let url = [ORIGIN, endpoint, params].join("");
        println!("{}", url);
        let resp = reqwest::blocking::get(url).unwrap();
        let status = resp.status();
        let resp_json = resp.json::<T>().unwrap();
        (status, resp_json)
    }
}
pub mod server;
pub mod simple;

//     pub mod coins {}
//     pub mod contract {}
//     pub mod asset_platform {}
//     pub mod categories {}
//     pub mod exchanges {}
//     pub mod indexes {}
//     pub mod derivatives {}
//     pub mod exchange_rates {}
//     pub mod search {}
//     pub mod trending {}
//     pub mod global {}
//     pub mod companies {}
