use sauron::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen(module = "/js/near-wrap.js")]
extern "C" {

    fn get_account_id() -> String;

    #[wasm_bindgen(catch)]
    async fn contract_get_num() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn contract_increment() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn contract_decrement() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    async fn contract_reset() -> Result<JsValue, JsValue>;

    fn sign_out();

    fn request_sign_in();
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
    /// the reset button is clicked
    ResetClicked,
    /// the contract counter successfully incremented
    ContractIncremented,
    /// the contract counter successfully decremented
    ContractDecremented,
    /// the contract counter successfully reset
    ContractReset,
    ToggleLeftEye,
    ToggleRightEye,
    ToggleLightIndicator,
    SignOutClicked,
    SignInClicked,
}

struct App {
    val: Option<i32>,
    loading: bool,
    left_eye: bool,
    right_eye: bool,
    light_indicator: bool,
}

impl App {
    fn new() -> Self {
        App {
            val: None,
            loading: false,
            left_eye: true,
            right_eye: false,
            light_indicator: false,
        }
    }

    fn update_ui() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                match contract_get_num().await {
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

    fn reset() -> Cmd<Self, Msg> {
        Cmd::new(|program| {
            spawn_local(async move {
                match contract_reset().await {
                    Ok(_) => {
                        log::trace!("contract reset");
                        program.dispatch(Msg::ContractReset);
                    }
                    Err(e) => {
                        program.dispatch(Msg::ContractError(e));
                    }
                }
            })
        })
    }

    fn view_signin_button(&self) -> Node<Msg> {
        node! {
             <div class="sign-in" on_click=|_|Msg::SignInClicked >
               <p>"You'll need to sign in to call contract methods:"</p>
               <button class="btn btn-primary" style="background-color: #0072CE;">"Sign In"</button>
             </div>
        }
    }

    fn view_signout_button(&self) -> Node<Msg> {
        node! {
              <div class="sign-out" on_click=|_|Msg::SignOutClicked >
                <button class="btn btn-primary" style="background-color: #0072CE;">"Sign Out"</button>
              </div>
        }
    }

    fn main_view(&self) -> Node<Msg> {
        let positive_cnt = if let Some(val) = self.val {
            val >= 0
        } else {
            false
        };

        let within_range = if let Some(val) = self.val {
            val > 20 || val < -20
        } else {
            false
        };
        node! {
              <div class="after-sign-in">
                  <div class="scene">
                    <div class="gameboy">
                      <div class="body-shape shadow"></div>
                      <div class="body-shape side"></div>
                      <div class="body-shape front">
                        <div class="screen">
                          <div class="dot" {classes_flag([("on", self.light_indicator)])}></div>
                          <div class="face">
                            <div class="eyes-row">
                              <div id="left" class="closed" {classes_flag([("eye",self.left_eye)])}>
                                <div class="pupil"></div>
                              </div>
                              <div id="right" class="closed" {classes_flag([("eye",self.right_eye)])}>
                                <div class="pupil"></div>
                              </div>
                            </div>
                            <div class="mouth-row">
                              <div class="mouth" {classes_flag([("smile", positive_cnt), ("cry", !positive_cnt)])}></div>
                              {
                                view_if(within_range,
                                  node!{
                                    <div class="tongue"></div>
                                    }
                                )
                             }
                            </div>
                          </div>
                          <div id="show" {classes_flag([ ("loader", self.loading),("number", !self.loading)])}>
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
                            <button id="plus" class="arrows" disabled={self.loading} on_click=|_|Msg::IncrementClicked>
                              <div class="left">
                              </div>
                              <div class="updown">
                              </div>
                            </button>
                            <button id="minus" class="arrows" disabled={self.loading} on_click=|_|Msg::DecrementClicked>
                              <div class="right">
                              </div>
                            </button>
                          </div>
                          <div class="selects row">
                            <div class="ab">
                              <div id="a" class="r a" on_click=|_|Msg::ResetClicked>"RS"</div>
                              <div id="b" class="r b" on_click=|_|Msg::ToggleRightEye>"LE"</div>
                              <div id="c" class="r c" on_click=|_|Msg::ToggleLeftEye>"RE"</div>
                              <div id="d" class="r d" on_click=|_|Msg::ToggleLightIndicator>"L"</div>
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>

                  { self.view_signout_button() }

              </div>
        }
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        self.loading = true;
        Self::update_ui()
    }

    fn view(&self) -> Node<Msg> {
        let signed_in = !get_account_id().is_empty();

        node! {
            <div class="container">
              <h1>"This is just a counter, but this time on blockchain!"</h1>

              { if !signed_in {
                    self.view_signin_button()
                }else{
                    self.main_view()
                }
              }

            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        log::trace!("dispatching msg: {:?}", msg);
        match msg {
            Msg::ReceivedCount(val) => {
                self.loading = false;
                self.val = Some(val);
                Cmd::none()
            }
            Msg::ContractError(e) => {
                log::error!("Something went wrong! {:?}", e);
                Cmd::none()
            }
            Msg::IncrementClicked => {
                self.loading = true;
                Self::increment()
            }
            Msg::DecrementClicked => {
                self.loading = true;
                Self::decrement()
            }
            Msg::ResetClicked => {
                self.loading = true;
                Self::reset()
            }
            Msg::ContractIncremented | Msg::ContractDecremented | Msg::ContractReset => {
                Self::update_ui()
            }
            Msg::ToggleLeftEye => {
                self.left_eye = !self.left_eye;
                Cmd::none()
            }
            Msg::ToggleRightEye => {
                self.right_eye = !self.right_eye;
                Cmd::none()
            }
            Msg::ToggleLightIndicator => {
                self.light_indicator = !self.light_indicator;
                Cmd::none()
            }
            Msg::SignOutClicked => {
                sign_out();
                Cmd::none()
            }
            Msg::SignInClicked => {
                request_sign_in();
                Cmd::none()
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    log::trace!("Hello from client rust");
    Program::mount_to_body(App::new());
}
