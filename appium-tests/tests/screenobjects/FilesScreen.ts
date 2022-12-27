import AppScreen from "./AppScreen"
import { customPredicateString } from "./../helpers/commands"
import { getPredicateForTextValueEqual } from "../helpers/commands"
import CreatePinScreen from "./CreatePinScreen"
import CreateAccountScreen from "./CreateAccountScreen"
import UplinkMainScreen from "./UplinkMainScreen"

const SELECTORS = {
  MACOS: {
    WINDOW: "-ios class chain:**/XCUIElementTypeWindow",
    FILES_TITLE: customPredicateString("48", "value", "Files"),
    FOLDER_NAME: customPredicateString("48", "value", "Folder 1"),
    SUBFOLDER_ONE_NAME: customPredicateString("48", "value", "Subdir1"),
    SUBFOLDER_TWO_NAME: customPredicateString("48", "value", "Subdir2"),
    SUBFOLDER_THREE_NAME: customPredicateString("48", "value", "f3"),
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
    DIRECTORY_TREE_ELEMENTS:
      "-ios class chain:**/XCUIElementTypeWebView/XCUIElementTypeStaticText",
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

  async loginToMainScreen(
    pin: string = "1234" + "\n",
    username: string = "filestest01" + "\n",
  ) {
    await CreatePinScreen.waitForIsShown(true)
    await (await CreatePinScreen.pinInput).setValue(pin)
    await CreateAccountScreen.waitForIsShown(true)
    await (await CreateAccountScreen.userInput).setValue(username)
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

  async validateSubfoldersDisplayed(foldersToDisplay: [String]) {
    for (let folder of foldersToDisplay) {
      const folderPredicate = $(getPredicateForTextValueEqual(folder))
      await folderPredicate.waitForDisplayed()
      await expect(folderPredicate).toHaveTextContaining(folder)
    }
  }

  async validateSubfoldersNotExisting(foldersNotExisting: [String]) {
    for (let folder of foldersNotExisting) {
      await expect(await $(getPredicateForTextValueEqual(folder))).not.toExist()
    }
  }
}

export default new FilesScreen()
