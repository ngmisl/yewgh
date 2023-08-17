use yew::prelude::*;
use yew_router::prelude::*;

// ===================================================================================
// for {username}.github.io/{repo_name}
// replace 'yew-template-for-github.io' to your repo name

#[derive(Clone, Routable, PartialEq)]
enum RootRoute {
    #[at("/yew-template-for-github-io/")]
    Home,
    #[at("/yew-template-for-github-io/:s")]
    Route,
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/yew-template-for-github-io/about")]
    About,
    #[not_found]
    #[at("/yew-template-for-github-io/404")]
    NotFound,
}

fn about() -> String {
    let abouturl = "/yew-template-for-github-io/about";
    return abouturl.to_string();
}

fn headerlinks() -> String {
    let homelink: &str = "/yew-template-for-github-io/";
    return homelink.to_string();
}

fn head() -> Html {
    let header = html! {<><p class="container flex flex-col items-center text-4xl"><a href={headerlinks()}> { "Home" }</a></p>
    </>};
    return header;
}

fn game() -> Html {
    let play = html! {
        <>
        <div class="container flex flex-col items-center justify-center px-5 mx-auto my-8">
        <div class="flex justify-center items-center h-screen">
          <iframe
            src="https://gateway.lighthouse.storage/ipfs/QmVEfSLJcGWussZ3ertnCNA9rYLDt47Yg7DoA4cL4eVnri/"
            width="1100"
            height="700"></iframe>
        </div>
      </div>
      </>
    };
    return play;
}

fn root_route(routes: &RootRoute) -> Html {
    match routes {
        RootRoute::Home => {
            html! { <>
                <div>{head()}</div>
                <div>{game()}</div>

            </>}
        }
        RootRoute::Route => html! {
            <Switch<Route> render={Switch::render(switch)} />
        },
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

// ===================================================================================
// for {username}.github.io

// #[derive(Clone, Routable, PartialEq)]
//  enum RootRoute {
//      #[at("/")]
//      Home,
//      #[at("/about")]
//      About,
//      #[not_found]
//      #[at("/404")]
//      NotFound,
//  }

//  fn root_route(routes: &Route) -> Html {
//      match routes {
//          RootRoute::Home => html! { <p class="text-4xl">{ "Yew Template" }</p> },
//          RootRoute::About => html! { <p>{ "About" }</p> },
//          RootRoute::NotFound => html! { <p>{ "Not Found" }</p> },
//      }
//  }

// ===================================================================================

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        // ********************************************************
        // **    basename is not supported on yew 0.19.0 yet.    **
        // <BrowserRouter basename="/yew-template-for-github-io/">
        //     <Switch<Route> render={Switch::render(switch)} />
        // </BrowserRouter>
        // ********************************************************
        <BrowserRouter>
            <Switch<RootRoute> render={Switch::render(root_route)} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::start_app::<App>();
}
