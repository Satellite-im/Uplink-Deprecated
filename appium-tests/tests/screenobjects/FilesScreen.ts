import AppScreen from "./AppScreen"
import { customPredicateString } from "./../helpers/commands"
import { getPredicateForTextValueEqual } from "../helpers/commands"
import EnterPinScreen from "./EnterPinScreen"
import UplinkMainScreen from "./UplinkMainScreen"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    FILES_TITLE: getPredicateForTextValueEqual("Files"),
    FOLDER_NAME: getPredicateForTextValueEqual("Folder 1"),
    SUBFOLDER_ONE_NAME: getPredicateForTextValueEqual("Subdir1"),
    SUBFOLDER_TWO_NAME: getPredicateForTextValueEqual("Subdir2"),
    SUBFOLDER_THREE_NAME: getPredicateForTextValueEqual("f3"),
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
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeButton[6]',
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
    DIRECTORY_TREE_ELEMENTS:
      '-ios class chain:**/XCUIElementTypeWebView/XCUIElementTypeStaticText[`value != "0 bytes / 0 item(s)"`]',
    HOME_BUTTON:
      '-ios class chain:**/XCUIElementTypeWebView[`label == "Dioxus app"`]/XCUIElementTypeGroup[7]/XCUIElementTypeGroup',
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

  get directoryTreeElements() {
    return $$(SELECTORS.MACOS.DIRECTORY_TREE_ELEMENTS)
  }

  get homeButton() {
    return $(SELECTORS.MACOS.HOME_BUTTON)
  }

  async loginToMainScreen(pin: string) {
    await EnterPinScreen.waitForIsShown(true)
    await EnterPinScreen.enterPin(pin)
    await UplinkMainScreen.waitForIsShown(true)
  }

  async goToFilesScreen() {
    await this.filesButton.click()
    await this.filesTitle.waitForDisplayed()
  }

  async waitForElementsLoaded() {
    await this.folderName.waitForDisplayed()
    await this.availableSpaceIndicatorText.waitForDisplayed()
    await this.usedSpaceIndicatorText.waitForDisplayed()
  }

  async clickOnDirectoryTreeElement(folderToClick: WebdriverIO.Element) {
    await folderToClick.click()
  }

  async createFolder(folderName: String) {
    await this.addFolderButton.click()
    await $("~New Folder").setValue(folderName + "\n")
  }

  async enterFolder(folderName: String) {
    const folder = await $("~" + folderName)
    await folder.click()
  }

  async goToHome() {
    await this.homeButton.click()
  }

  async goToParentFolder(folderName: String) {
    await $("~" + folderName).click()
  }

  async goToFolder(folderName: String) {}

  async renameFolder(currentName: String, newName: String) {}

  async deleteFolder(folderName: String) {}

  async uploadFile() {}

  async downloadFile() {}

  async moveFileToSubfolder(fileName: String, subfolderName: String) {}
}

export default new FilesScreen()
