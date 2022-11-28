import AppScreen from "../screenobjects/AppScreen"

const SELECTORS = {
  MACOS: {
    HEADER_TEXT: '-ios predicate string:value == "Create Account"',
    SUBTITLE_TEXT:
      "-ios class chain:**/XCUIElementTypeStaticText[`value == \"It's free and fast, just tell us what you'd like your username to be.\"`][1]",
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    PROFILE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[3]',
    USERNAME_INPUT:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[5]/XCUIElementTypeTextField',
    CREATE_ACCOUNT_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton',
    ERROR_MESSAGE:
      "//XCUIElementTypeWebView/XCUIElementTypeGroup[6]/XCUIElementTypeStaticText",
  },
}

class CreateAccountScreen extends AppScreen {
  constructor() {
    super("~Create a Pin")
  }

  get headerText() {
    return $(SELECTORS.MACOS.HEADER_TEXT)
  }

  get window() {
    return $(SELECTORS.MACOS.WINDOW)
  }

  get subtitleText() {
    return $(SELECTORS.MACOS.SUBTITLE_TEXT)
  }

  get profileButton() {
    return $(SELECTORS.MACOS.PROFILE_BUTTON)
  }

  get userInput() {
    return $(SELECTORS.MACOS.USERNAME_INPUT)
  }

  get errorMessage() {
    return $(SELECTORS.MACOS.ERROR_MESSAGE)
  }

  get createAccountButton() {
    return $(SELECTORS.MACOS.CREATE_ACCOUNT_BUTTON)
  }
}

export default new CreateAccountScreen()
