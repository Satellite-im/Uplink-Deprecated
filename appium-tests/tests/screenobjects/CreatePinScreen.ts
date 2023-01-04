import AppScreen from "./AppScreen"
import { customPredicateString } from "../helpers/commands"
import { getPredicateForTextValueEqual } from "../helpers/commands"

const SELECTORS = {
  MACOS: {
    HEADER_TEXT: '-ios predicate string:value == "Create a Pin"',
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    PIN_INPUT:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeTextField',
    SUBTITLE_TEXT:
      '-ios class chain:**/XCUIElementTypeStaticText[`value == "Choose a 4-6 digit pin to secure your account."`][1]',
    PROFILE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[1]',
    WORLD_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[2]',
    ERROR_MESSAGE_INVALID_PIN: getPredicateForTextValueEqual(
      "Your pin must be at least 4 characters.　",
    ),
    MAX_LENGTH_TEXT: customPredicateString(
      "9",
      "title",
      "Only four to six characters allowed",
      "==",
    ),
  },
}

class CreatePinScreen extends AppScreen {
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

  async enterPin(pin: string = "") {
    await this.pinInput.setValue(pin)
  }

  async resetApp() {
    await driver.reset()
    await this.headerText.waitForDisplayed()
  }
}

export default new CreatePinScreen()
