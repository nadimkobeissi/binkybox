extern crate embed_resource;

fn main() {
	embed_resource::compile("binkybox-manifest.rc", embed_resource::NONE);
}
