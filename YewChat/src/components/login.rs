use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div
            class="flex w-screen h-screen items-center justify-center"
            style="background-color: #FAF1E6;"
        >
            <div
                class="p-8 rounded-lg shadow-lg"
                style="background-color: #FDFAF6; max-width: 400px; width: 100%; text-align: center;"
            >
                <h1
                    class="text-2xl font-bold mb-4"
                    style="color: #99BC85;"
                >
                    {"🗨️ Welcome to KayChat!"}
                </h1>

                <p
                    class="mb-6"
                    style="color: #4A4A4A;"
                >
                    {"Enter a cool username and dive into the fun."}
                </p>

                <form class="flex">
                    <input
                        {oninput}
                        placeholder="Your username"
                        class="flex-grow rounded-l-lg p-3 outline-none"
                        style="
                            background-color: #E4EFE7;
                            border: 1px solid #FAF1E6;
                            color: #333;
                        "
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len() < 1}
                            class="px-6 rounded-r-lg font-semibold"
                            style="
                                background-color: #99BC85;
                                border: 1px solid #99BC85;
                                color: #FDFAF6;
                            "
                        >
                            {"Start Chatting"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}