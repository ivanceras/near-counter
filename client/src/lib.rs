use sauron::html::attributes::style;
use sauron::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(module = "/js/near-wrap.js")]
extern "C" {
    fn getAccountId() -> String;
    #[wasm_bindgen(catch)]
    async fn contractGetNum() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn contract_increment() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn contract_decrement() -> Result<JsValue, JsValue>;
}

#[derive(Debug)]
enum Msg {
    /// received the contract current counter value
    ReceivedCount(i32),
    /// there was an error somewhere along when calling the api contract
    ContractError(JsValue),
    /// the increment button is clicked
    IncrementClicked,
    /// the decrement button is clicked
    DecrementClicked,
    /// the contract counter successfully incremented
    ContractIncremented,
    /// the contract counter successfully decremented
    ContractDecremented,
}

struct App {
    val: Option<i32>,
}

impl App {
    fn new() -> Self {
        App { val: None }
    }

    fn update_ui() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                match contractGetNum().await {
                    Ok(number) => {
                        let number = number.as_f64().expect("must be a number");
                        log::trace!("got value: {}", number);
                        program.dispatch(Msg::ReceivedCount(number as i32));
                    }
                    Err(e) => {
                        log::trace!("error getting value: {:?}", e);
                        program.dispatch(Msg::ContractError(e));
                    }
                }
            })
        })
    }

    fn increment() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                match contract_increment().await {
                    Ok(_) => {
                        log::trace!("incremented contract");
                        program.dispatch(Msg::ContractIncremented);
                    }
                    Err(e) => {
                        program.dispatch(Msg::ContractError(e));
                    }
                }
            })
        })
    }

    fn decrement() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                match contract_decrement().await {
                    Ok(_) => {
                        log::trace!("decremented contract");
                        program.dispatch(Msg::ContractDecremented);
                    }
                    Err(e) => {
                        program.dispatch(Msg::ContractError(e));
                    }
                }
            })
        })
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Self::update_ui()
    }

    fn view(&self) -> Node<Msg> {
        let signed_in = !getAccountId().is_empty();
        node! {
        <div class="container">
          <h1>"This is just a counter, but this time on blockchain!"</h1>

          <div class="sign-in" {style("display", if !signed_in{"block"}else{"none"})}>
            <p>"You'll need to sign in to call contract methods:"</p>
            <button class="btn btn-primary" style="background-color: #0072CE;">"Sign In"</button>
          </div>

          <div class="after-sign-in" {style("display", if signed_in{"block"}else{"none"})}>
              <div class="scene">
                <div class="gameboy">
                  <div class="body-shape shadow"></div>
                  <div class="body-shape side"></div>
                  <div class="body-shape front">
                    <div class="screen">
                      <div class="dot"></div>
                      <div class="face">
                        <div class="eyes-row">
                          <div id="left" class="closed">
                            <div class="pupil"></div>
                          </div>
                          <div id="right" class="closed">
                            <div class="pupil"></div>
                          </div>
                        </div>
                        <div class="mouth-row">
                          <div class="mouth smile"></div>
                          <div class="tongue"></div>
                        </div>
                      </div>
                      <div id="show" {if let Some(_val)=self.val{class("number")}else{class("loader")}}>
                        {
                            if let Some(val) = self.val{
                                text(val)
                            }else{
                                text("calculating...")
                            }
                        }
                      </div>
                    </div>
                    <div class="buttons">
                      <div class="row">
                        <button id="plus" class="arrows" on_click=|_|Msg::IncrementClicked>
                          <div class="left">
                          </div>
                          <div class="updown">
                          </div>
                        </button>
                        <button id="minus" class="arrows" on_click=|_|Msg::DecrementClicked>
                          <div class="right">
                          </div>
                        </button>
                      </div>
                      <div class="selects row">
                        <div class="ab">
                          <div id="a" class="r a">"RS"</div>
                          <div id="b" class="r b">"LE"</div>
                          <div id="c" class="r c">"RE"</div>
                          <div id="d" class="r d">"L"</div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div class="sign-out">
                <button class="btn btn-primary" style="background-color: #0072CE;">"Sign Out"</button>
              </div>
          </div>
        </div>
          }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        log::trace!("dispatching msg: {:?}", msg);
        match msg {
            Msg::ReceivedCount(val) => {
                self.val = Some(val);
                Cmd::none()
            }
            Msg::ContractError(e) => {
                log::error!("Something went wrong! {:?}", e);
                Cmd::none()
            }
            Msg::IncrementClicked => {
                self.val = None;
                Self::increment()
            }
            Msg::DecrementClicked => {
                self.val = None;
                Self::decrement()
            }
            Msg::ContractIncremented => Self::update_ui(),
            Msg::ContractDecremented => Self::update_ui(),
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    log::trace!("Hello from client rust");
    log::debug!(" account_id {}", getAccountId());
    Program::mount_to_body(App::new());
}
