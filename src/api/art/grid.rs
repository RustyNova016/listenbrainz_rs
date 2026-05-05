use api_bindium::ApiRequest;
use api_bindium::ApiRequestError;
use api_bindium::HTTPVerb;
use api_bindium::Parser;
use api_bindium::TextParser;
use api_bindium::api_response::ureq_response::UreqResponseInner;
use api_bindium::endpoints::UriBuilderError;
use api_bindium::ureq;
use base64::Engine as _;
use base64::prelude::BASE64_STANDARD;
use regex::Regex;
use resvg::tiny_skia::Pixmap;
use resvg::usvg::Options;
use resvg::usvg::Tree;

use crate::api::ListenBrainzAPIEnpoints;

impl ListenBrainzAPIEnpoints {
    pub fn post_art_grid(
        &self,
        body: ArtGridQuery,
    ) -> Result<ApiRequest<SVGParser>, UriBuilderError> {
        println!("{}", serde_json::to_value(&body).unwrap());

        let request = self
            .endpoint_builder()
            .set_path("/1/art/grid/")
            .into_api_request_with_body(
                HTTPVerb::Post,
                serde_json::to_value(&body).unwrap(),
                SVGParser,
            )?;

        Ok(request)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, bon::Builder)]
pub struct ArtGridQuery {
    /// The background for the cover art
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<ArtGridQueryBackground>,

    /// The size of the cover art image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_size: Option<u32>,

    /// The dimension to use for this grid. A grid of dimension 3 has 3 images across and 3 images down, for a total of 9 images.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimension: Option<u8>,

    /// If cover art is missing for a given release_mbid, skip it and move on to the next one, if true is passed. If false, the show-caa option will decide what happens.
    #[serde(rename = "skip-missing", skip_serializing_if = "Option::is_none")]
    pub skip_missing: Option<bool>,

    /// If the cover art is missing and skip-missing is false, then show-caa will determine if a blank square is shown or if the Cover Art Archive missing image is shown.
    #[serde(rename = "show-caa", skip_serializing_if = "Option::is_none")]
    pub show_caa: Option<bool>,

    /// Whether to show the release name and artist overlayed on each cover art image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<bool>,

    /// The tiles paramater is a list of strings that determines the location where cover art images should be placed. Each string is a comma separated list of image cells. A grid of dimension 3 has 9 cells, from 0 in the upper left hand corner, 2 in the upper right hand corner, 6 in the lower left corner and 8 in the lower right corner. Specifying only a single cell will have the image cover that cell exactly. If more than one cell is specified, the image will cover the area defined by the bounding box of all the given cells. These tiles only define bounding box areas – no clipping of images that may fall outside of these tiles will be performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<Vec<String>>,

    /// An ordered list of release_mbids. The images will be loaded and processed in the order that this list is in. The cover art for the release_mbids will be placed on the tiles defined by the tiles parameter. If release_group_mbids are supplied as well, ONLY cover arts for release_group_mbids will be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_mbids: Option<Vec<String>>,
    /// An ordered list of release_group_mbids. The images will be loaded and processed in the order that this list is in. The cover art for the release_group_mbids will be placed on the tiles defined by the tiles parameter. If release_mbids are supplied as well, ONLY cover arts for release_mbids will be processed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_group_mbids: Option<Vec<String>>,

    /// Size in pixels of each cover art in the composited image. Can be either 250 or 500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art_size: Option<u32>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ArtGridQueryBackground {
    Transparent,
    White,
    Black,
}

pub struct SVGParser;

impl Parser<UreqResponseInner> for SVGParser {
    type Error = ApiRequestError;
    type Output = Pixmap;

    fn parse(&self, response: UreqResponseInner) -> Result<Self::Output, Self::Error> {
        let text = TextParser.parse(response)?;
        let text = inbed_images(&text);

        let options = Options::default();
        let tree = Tree::from_str(&text, &options).unwrap();

        let pixmap_size = tree.size().to_int_size();
        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();

        resvg::render(
            &tree,
            resvg::usvg::Transform::default(),
            &mut pixmap.as_mut(),
        );

        Ok(pixmap)
    }
}

fn inbed_images(svg_text: &str) -> String {
    // Find all the links
    let regex =
        Regex::new(r#"(?m)href="(https?:\/\/archive\.org\/download\/mbid.*)\.(..?.?.?)""#).unwrap();
    let mut new_text = svg_text.to_string();

    for (_, [url, ext]) in regex.captures_iter(&svg_text).map(|c| c.extract()) {
        // Convert all the links to data urls
        let mut response = ureq::get(&format!("{url}.{ext}")).call().unwrap();
        let bytes = response.body_mut().with_config().read_to_vec().unwrap();
        let data_url = format!("data:image/{ext};base64,{}", BASE64_STANDARD.encode(&bytes));
        new_text = new_text.replace(&format!("{url}.{ext}"), &data_url);
    }

    new_text
}



#[cfg(test)]
mod test {
    use crate::ListenBrainzClient;
    use crate::api::art::grid::ArtGridQuery;

    #[test]
    pub fn art_grid_test() {
        let body = ArtGridQuery {
            background: Some(super::ArtGridQueryBackground::Transparent),
            image_size: Some(750),
            dimension: Some(4),
            skip_missing: Some(false),
            show_caa: Some(false),
            caption: Some(false),
            tiles: Some(vec![
                "0,1,4,5".to_string(),
                "10,11,14,15".to_string(),
                "2".to_string(),
                "3".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "12".to_string(),
                "13".to_string(),
            ]),
            release_mbids: Some(vec![
                "d101e395-0c04-4237-a3d2-167b1d88056c".to_string(),
                "4211382c-39e8-4a72-a32d-e4046fd96356".to_string(),
                "6d895dfa-8688-4867-9730-2b98050dae04".to_string(),
                "773e54bb-3f43-4813-826c-ca762bfa8318".to_string(),
                "ec782dbe-9204-4ec3-bf50-576c7cf3dfb3".to_string(),
                "10dffffc-c2aa-4ddd-81fd-42b5e125f240".to_string(),
                "be5f714d-02eb-4c89-9a06-5e544f132604".to_string(),
                "3eee4ed1-b48e-4894-8a05-f535f16a4985".to_string(),
            ]),
            cover_art_size: None,
            release_group_mbids: None,
        };

        let client = ListenBrainzClient::default();

        client
            .endpoints()
            .post_art_grid(body)
            .unwrap()
            .send(&client.api_client())
            .unwrap()
            .parse()
            .unwrap();
    }
}
