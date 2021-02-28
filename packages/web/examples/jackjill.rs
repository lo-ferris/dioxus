use dioxus::prelude::*;
use dioxus_core as dioxus;
use dioxus_web::WebsysRenderer;

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(WebsysRenderer::start(Example))
}

static Example: FC<()> = |ctx, props| {
    let (name, set_name) = use_state(&ctx, || "...?");

    log::debug!("Running component....");

    ctx.view(html! {
    // <div>
    //     <h1> "Hello, {name}" </h1>
    //     <button onclick={move |_| set_name("jack")}> "jack!" </button>
    //     <button
    //         onclick={move |_| set_name("jill")}
    //         onclick={move |_| set_name("jill")}
    //     > "jill!" </button>
    // </div>
            <div>
                <section class="py-12 px-4 text-center">
                    <div class="w-full max-w-2xl mx-auto">
                        // Tagline
                        <span class="text-sm font-semibold">
                            "Dioxus Example: Jack and Jill"
                        </span>

                        // Header
                        <h2 class="text-5xl mt-2 mb-6 leading-tight font-semibold font-heading">
                            "Hello, {name}"
                        </h2>

                        // Control buttons
                        <div>
                            <button
                                class="inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
                                // onclick={move |_| log::debug!("set jack")}>
                                onclick={move |_| set_name("jack")}>
                                "Jack!"
                                </button>

                                <button
                                class="inline-block py-4 px-8 mr-6 leading-none text-white bg-indigo-600 hover:bg-indigo-900 font-semibold rounded shadow"
                                // onclick={move |_| log::debug!("set jill")}>
                                onclick={move |_| set_name("jill")}
                                onclick={move |_| set_name("jill")}>
                                "Jill!"
                            </button>
                        </div>
                    </div>
                </section>
            </div>
        })
};

// <div>
//     <h1> "Hello, {name}" </h1>
//     <button onclick={move |_| set_name("jack   .")}> "jack!" </button>
//     <button onclick={move |_| set_name("jill   .")}> "jill!" </button>
// </div>