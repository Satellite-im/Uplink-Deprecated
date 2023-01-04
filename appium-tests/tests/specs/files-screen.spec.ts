import FilesScreen from "../screenobjects/FilesScreen"
import { getPredicateForTextValueEqual } from "../helpers/commands"

describe("Files Screen Tests on Uplink Desktop", async () => {
  before(async () => {
    // Create an account and go to Main Screen
    await FilesScreen.loginToMainScreen("123456" + "\n")
  })

  it("Go to Files Screen and validate text contents", async () => {
    // Click on Files Button and assert contents on screen are correct
    await FilesScreen.goToFilesScreen()
    await FilesScreen.waitForElementsLoaded()
    await expect(await FilesScreen.filesTitle).toHaveTextContaining("Files")
    await expect(await FilesScreen.folderName).toHaveTextContaining("Folder 1")
    await expect(
      await FilesScreen.availableSpaceIndicatorText,
    ).toHaveTextContaining(/[. 0-9] Free/i)
    await expect(await FilesScreen.usedSpaceIndicatorText).toHaveTextContaining(
      [/[. 0-9]+(KB|MB|GB) /, "/ ", /. 0-9]+(KB|MB|GB)/i],
    )
  })

  it("Click on Folder 1 and validate that subfolders are displayed", async () => {
    // Click on main Folder from directory tree
    await FilesScreen.clickOnDirectoryTreeElement(FilesScreen.folderName)
    const foldersToDisplay = ["Subdir1", "Subdir2", "f3"]
    for (let folder of foldersToDisplay) {
      const folderPredicate = $(getPredicateForTextValueEqual(folder))
      await folderPredicate.waitForDisplayed()
      await expect(folderPredicate).toHaveTextContaining(folder)
    }
  })

  it("Files Directory Tree - Display the whole tree", async () => {
    // Locate subfolder elements and click on them
    const subdir1 = await $(getPredicateForTextValueEqual("Subdir1"))
    const subdir2 = await $(getPredicateForTextValueEqual("Subdir2"))
    const subdir3 = await $(getPredicateForTextValueEqual("Subdir3"))
    await FilesScreen.clickOnDirectoryTreeElement(subdir1)
    await FilesScreen.clickOnDirectoryTreeElement(subdir2)
    await FilesScreen.clickOnDirectoryTreeElement(subdir3)

    // Assert subfolders are displayed and texts are matching
    const foldersToDisplay = ["f1", "f2", "f3"]
    for (let folder of foldersToDisplay) {
      const folderPredicate = $(getPredicateForTextValueEqual(folder))
      await folderPredicate.waitForDisplayed()
      await expect(folderPredicate).toHaveTextContaining(folder)
    }

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(7)
  })

  it("Files Directory Tree - Hide children elements", async () => {
    // Locate Subdir2 element and click on it
    const subdir2 = await $(getPredicateForTextValueEqual("Subdir2"))
    await FilesScreen.clickOnDirectoryTreeElement(subdir2)

    // Assert subfolders from Subdir2 does not exist in screen
    const foldersNotExisting = ["Subdir3", "f2"]
    for (let folder of foldersNotExisting) {
      await expect(await $(getPredicateForTextValueEqual(folder))).not.toExist()
    }

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(5)
  })

  it("Files Directory Tree - Hide all the tree", async () => {
    // Locate Main Folder element and click on it
    const folder1 = await $(getPredicateForTextValueEqual("Folder 1"))
    await FilesScreen.clickOnDirectoryTreeElement(folder1)

    // Assert subfolders from Subdir2 does not exist in screen
    const foldersNotExisting = ["Subdir1", "f1", "Subdir2", "f2"]
    for (let folder of foldersNotExisting) {
      await expect(await $(getPredicateForTextValueEqual(folder))).not.toExist()
    }

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(1)
  })

  it("Files Navigation - Go to Home when no folders are created", async () => {
    await FilesScreen.goToHome()
    await FilesScreen.homeButton.click()
    const myLocator = await FilesScreen.homeButton.$$("~>")
    await expect(myLocator).not.toExist()
  })

  it("Files Navigation - Create a new folder", async () => {
    await FilesScreen.createFolder("test01")
    await expect(await $("~0 bytes")).toBeDisplayed()
    await expect(await $("~0 item(s)")).toBeDisplayed()
  })

  xit("Files Navigation - Create a folder with same name than other existing in same location", async () => {})

  xit("Files Navigation - Rename a folder", async () => {})

  it("Files Navigation - Navigate into a subfolder", async () => {
    await FilesScreen.enterFolder("test01")
  })

  xit("Files Navigation - Navigate into a parent folder", async () => {})

  xit("Files Navigation - Navigate into main directory", async () => {})

  xit("Files Navigation - Delete a folder", async () => {})

  xit("Files - Upload a file", async () => {})

  xit("Files - Upload multiple files at the same time", async () => {})

  xit("Files - Upload a file with same filename", async () => {})

  xit("Files - Click on upload file but then cancel operation", async () => {})

  xit("Files - Cancel a file upload while file is being uploaded", async () => {})

  xit("Files - Rename a file", async () => {})

  xit("Files - Delete a file", async () => {})

  xit("Files - Download a File", async () => {})

  xit("Files - Move a file into a subfolder", async () => {})

  xit("Files - Upload a file and validate that space indicators from above are updated", async () => {})

  xit("Files - Upload a file and validate that subfolder space/item indicators are updated", async () => {})

  xit("Files - Upload a file and validate that main directory space/item indicators are updated", async () => {})
})
