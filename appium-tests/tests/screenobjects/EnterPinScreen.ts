import AppScreen from "./AppScreen"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    HEADER_TEXT: '-ios predicate string:value == "Enter Pin"',
    SUBTITLE_TEXT:
      '-ios class chain:**/XCUIElementTypeStaticText[`value == "Enter pin to unlock your account."`][1]',
    PIN_INPUT:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeTextField',
    PROFILE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[1]',
    WORLD_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[2]',
    ERROR_MESSAGE_INVALID_PIN:
      '//*[contains(@value, "Your pin must be at least 4 characters")]',
    MAX_LENGTH_TEXT: '//*[@title="Only four to six characters allowed"]',
  },
}

class EnterPinScreen extends AppScreen {
  constructor() {
    super(SELECTORS.MACOS.HEADER_TEXT)
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

  get worldButton() {
    return $(SELECTORS.MACOS.WORLD_BUTTON)
  }

  get invalidPinMessage() {
    return $(SELECTORS.MACOS.ERROR_MESSAGE_INVALID_PIN)
  }

  get pinInput() {
    return $(SELECTORS.MACOS.PIN_INPUT)
  }

  get maxLengthMessage() {
    return $(SELECTORS.MACOS.MAX_LENGTH_TEXT)
  }
}

export default new EnterPinScreen()
