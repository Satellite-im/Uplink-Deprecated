import AppScreen from "../screenobjects/AppScreen"

const SELECTORS = {
  MACOS: {
    HEADER_TEXT: '-ios predicate string:value == "Create Account"',
    SUBTITLE_TEXT:
      "-ios class chain:**/XCUIElementTypeStaticText[`value == \"It's free and fast, just tell us what you'd like your username to be.\"`][1]",
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    PROFILE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[3]',
    USERNAME_INPUT: "-ios class chain:**/XCUIElementTypeTextField",
    CREATE_ACCOUNT_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton',
    ERROR_MESSAGE_USERNAME:
      "//XCUIElementTypeWebView/XCUIElementTypeGroup[5]/XCUIElementTypeStaticText",
  },
}

class CreateAccountScreen extends AppScreen {
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

  get userInput() {
    return $(SELECTORS.MACOS.USERNAME_INPUT)
  }

  get errorMessageUsername() {
    return $(SELECTORS.MACOS.ERROR_MESSAGE_USERNAME)
  }

  get createAccountButton() {
    return $(SELECTORS.MACOS.CREATE_ACCOUNT_BUTTON)
  }

  async enterUsername(username: string = "") {
    await this.userInput.setValue(username + "\n")
  }

  async validateEmptyUsername() {
    await this.errorMessageUsername.waitForDisplayed()
    await expect(await this.errorMessageUsername).toHaveTextContaining(
      "Username is required",
    )
  }

  async validateUsernameWrongChars() {
    await this.errorMessageUsername.waitForDisplayed()
    await expect(await this.errorMessageUsername).toHaveTextContaining(
      "Username needs to be between 4 and 32 characters long",
    )
  }
}

export default new CreateAccountScreen()
