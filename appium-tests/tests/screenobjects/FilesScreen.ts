import AppScreen from "./AppScreen"
import { customPredicateString } from "./../helpers/commands"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    FILES_TITLE: customPredicateString("48", "value", "Files"),
    FOLDER_NAME: customPredicateString("48", "value", "Folder 1"),
    SUBFOLDER_ONE_NAME: customPredicateString("48", "value", "Subdir1"),
    SUBFOLDER_TWO_NAME: customPredicateString("48", "value", "Subdir2"),
    SUBFOLDER_THREE_NAME: customPredicateString("48", "value", "f3"),
    HOME_DIRECTORY_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[7]/XCUIElementTypeGroup',
    USED_SPACE_FOLDER_TEXT: customPredicateString(
      "48",
      "value",
      "bytes",
      "CONTAINS",
    ),
    AVAILABLE_SPACE_INDICATOR_BAR:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[3]',
    AVAILABLE_SPACE_INDICATOR_TEXT: customPredicateString(
      "48",
      "value",
      "Free",
      "CONTAINS",
    ),
    USED_SPACE_INDICATOR_BAR:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[4]',
    USED_SPACE_INDICATOR_TEXT: customPredicateString(
      "48",
      "value",
      "GB",
      "CONTAINS",
    ),
    DELETE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[5]',
    ADD_FOLDER_BUTTON:
      'ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[6]',
    ADD_FILE_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[7]',
    CHATS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[1]',
    FILES_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[2]',
    CONTACTS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[3]',
    SETTINGS_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[4]',
  },
}

class FilesScreen extends AppScreen {
  constructor() {
    super(SELECTORS.MACOS.FILES_TITLE)
  }

  get window() {
    return $(SELECTORS.MACOS.WINDOW)
  }

  get filesTitle() {
    return $(SELECTORS.MACOS.FILES_TITLE)
  }

  get folderName() {
    return $(SELECTORS.MACOS.FOLDER_NAME)
  }

  get homeDirectoryButton() {
    return $(SELECTORS.MACOS.HOME_DIRECTORY_BUTTON)
  }

  get usedSpaceFolderText() {
    return $$(SELECTORS.MACOS.USED_SPACE_FOLDER_TEXT)[0]
  }

  get availableSpaceIndicatorBar() {
    return $$(SELECTORS.MACOS.AVAILABLE_SPACE_INDICATOR_BAR)[0]
  }

  get availableSpaceIndicatorText() {
    return $$(SELECTORS.MACOS.AVAILABLE_SPACE_INDICATOR_TEXT)[0]
  }

  get usedSpaceIndicatorBar() {
    return $$(SELECTORS.MACOS.USED_SPACE_INDICATOR_BAR)[0]
  }

  get usedSpaceIndicatorText() {
    return $$(SELECTORS.MACOS.USED_SPACE_INDICATOR_TEXT)[0]
  }

  get deleteFolderButton() {
    return $(SELECTORS.MACOS.DELETE_BUTTON)
  }

  get addFolderButton() {
    return $(SELECTORS.MACOS.ADD_FOLDER_BUTTON)
  }

  get addFileButton() {
    return $(SELECTORS.MACOS.ADD_FILE_BUTTON)
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
}

export default new FilesScreen()