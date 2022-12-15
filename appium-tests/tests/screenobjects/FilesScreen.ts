import AppScreen from "./AppScreen"
import { customPredicateString } from "./../helpers/commands"

const regExAvailableSpace: string = "‘[. 0-9]+(KB|MB|GB) / [. 0-9]+(KB|MB|GB)’"
const regExUsedSpace: string = "‘[. 0-9] Free’"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    FILES_TITLE: customPredicateString("48", "value", "Files"),
    FOLDER_NAME: customPredicateString("48", "value", "Folder 1"),
    SUBFOLDER_ONE_NAME: customPredicateString("48", "value", "Subdir1"),
    SUBFOLDER_TWO_NAME: customPredicateString("48", "value", "Subdir2"),
    SUBFOLDER_THREE_NAME: customPredicateString("48", "value", "f3"),
    MAIN_DIRECTORY_TEXT: customPredicateString("48", "value", "main_directory"),
    AVAILABLE_SPACE_INDICATOR_BAR:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[3]',
    AVAILABLE_SPACE_INDICATOR_TEXT: `-ios predicate string:elementType == 48 AND value MATCHES ${regExAvailableSpace}`,
    USED_SPACE_INDICATOR_BAR:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[4]',
    USED_SPACE_INDICATOR_TEXT: `-ios predicate string:elementType == 48 AND value MATCHES ${regExUsedSpace}`,
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

  get mainDirectoryText() {
    return $(SELECTORS.MACOS.MAIN_DIRECTORY_TEXT)
  }

  get availableSpaceIndicatorBar() {
    return $(SELECTORS.MACOS.AVAILABLE_SPACE_INDICATOR_BAR)
  }

  get availableSpaceIndicatorText() {
    return $(SELECTORS.MACOS.AVAILABLE_SPACE_INDICATOR_TEXT)
  }

  get usedSpaceIndicatorBar() {
    return $(SELECTORS.MACOS.USED_SPACE_INDICATOR_BAR)
  }

  get usedSpaceIndicatorText() {
    return $(SELECTORS.MACOS.USED_SPACE_INDICATOR_TEXT)
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
