use warp::tesseract::Tesseract;
use yew::prelude::*;

use crate::language;


pub enum SupportedMessages {
    TryUnlock
}

pub struct UnlockComponent {
    pub locked: bool,
    pub unlock_phrase: String,
}

impl Component for UnlockComponent {
    type Message = SupportedMessages;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let tesseract = Tesseract::from_file(".warp_datastore");
        
        Self {
            locked: !tesseract.is_ok(),
            unlock_phrase: String::from("")
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SupportedMessages::TryUnlock => {
                println!("Try Unlock");
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let language = language::by_locale(language::AvailableLanguages::EN_US);

        html! {
            <div class="h-full flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
                <div class="max-w-md w-full space-y-8 align-middle ">
                    <div>
                        <h2 class="mt-6 text-center text-3xl tracking-tight font-bold text-gray-900">{language.unlock_title}</h2>
                        <p class="text-center text-gray-900">{language.unlock_desc}</p>
                    </div>
                    <form class="mt-8 space-y-6" action="#" method="POST">
                        <input type="hidden" name="remember" value="true" />
                        <div class="rounded-md shadow-sm -space-y-px">
                            <div>
                                <label for="password" class="sr-only">{"Enter your Pin"}</label>
                                <input
                                    id="password" 
                                    name="password" 
                                    type="password" 
                                    autocomplete="current-password" 
                                    required=true 
                                    class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                    placeholder={ format!("{}..", language.passphrase) } />
                            </div>
                        </div>
                        <div class="flex items-center justify-between">
                            <div class="flex items-center">
                                <input id="remember-me" name="remember-me" type="checkbox" class="h-4 w-4 text-indigo-600 focus:ring-indigo-500 border-gray-300 rounded" />
                                <label for="remember-me" class="ml-2 block text-sm text-gray-900"> {"Remember me"} </label>
                            </div>

                            <div class="text-sm">
                                <a href="#" class="font-medium text-indigo-600 hover:text-indigo-500"> { "Recover an Account?" }</a>
                            </div>
                        </div>
                        <div>
                            <button 
                                onclick={link.callback(|_| SupportedMessages::TryUnlock)}
                                class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                                <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                                        <svg class="h-5 w-5 text-indigo-500 group-hover:text-indigo-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clip-rule="evenodd" />
                                    </svg>
                                </span>
                                { language.unlock }
                            </button>
                        </div>
                    </form>
                </div>
            </div>
        }
    }
}