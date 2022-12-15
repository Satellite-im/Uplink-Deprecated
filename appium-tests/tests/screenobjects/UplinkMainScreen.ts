import AppScreen from "./AppScreen"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    SEARCH_BAR:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[3]/XCUIElementTypeTextField',
    FAVORITES_TEXT:
      '-ios class chain:**/XCUIElementTypeStaticText[`value == "Favorites"`][1]',
    NEW_TEXT: '-ios predicate string:value == "New"',
    CHATS_TEXT:
      '-ios class chain:**/XCUIElementTypeStaticText[`value == "Chats"`][1]',

    NO_ACTIVE_CHATS_TEXT:
      '-ios predicate string:value == "No active chats, wanna make one?"',
    CHATS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[1]',
    FILES_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[2]',
    CONTACTS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[3]',
    SETTINGS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[4]',
    START_ONE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[5]',
  },
}

class UplinkMainScreen extends AppScreen {
  constructor() {
    super(SELECTORS.MACOS.NO_ACTIVE_CHATS_TEXT)
  }

  get window() {
    return $(SELECTORS.MACOS.WINDOW)
  }

  get searchBar() {
    return $(SELECTORS.MACOS.SEARCH_BAR)
  }

  get favoritesText() {
    return $(SELECTORS.MACOS.FAVORITES_TEXT)
  }

  get newText() {
    return $(SELECTORS.MACOS.NEW_TEXT)
  }

  get chatsText() {
    return $(SELECTORS.MACOS.CHATS_TEXT)
  }

  get noActiveChatsText() {
    return $(SELECTORS.MACOS.NO_ACTIVE_CHATS_TEXT)
  }

  get chatsButton() {
    return $(SELECTORS.MACOS.CHATS_BUTTON)
  }

  get filesButton() {
    return $(SELECTORS.MACOS.FILES_BUTTON)
  }

  get contactsButton() {
    return $(SELECTORS.MACOS.CONTACTS_BUTTON)
  }

  get settingsButton() {
    return $(SELECTORS.MACOS.SETTINGS_BUTTON)
  }

  get startOneButton() {
    return $(SELECTORS.MACOS.START_ONE_BUTTON)
  }
}

export default new UplinkMainScreen()
