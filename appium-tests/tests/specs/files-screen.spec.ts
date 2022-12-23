import FilesScreen from "../screenobjects/FilesScreen"
import CreatePinScreen from "../screenobjects/CreatePinScreen"
import CreateAccountScreen from "../screenobjects/CreateAccountScreen"
import UplinkMainScreen from "../screenobjects/UplinkMainScreen"
import { getPredicateForTextValueEqual } from "../helpers/commands"

describe("Files Screen Tests on Uplink Desktop", async () => {
  before(async () => {
    // Create an account and go to Main Screen
    await CreatePinScreen.waitForIsShown(true)
    await (await CreatePinScreen.pinInput).setValue("1234" + "\n")
    await CreateAccountScreen.waitForIsShown(true)
    await (await CreateAccountScreen.userInput).setValue("filestest01" + "\n")
    await UplinkMainScreen.waitForIsShown(true)
  })

  it("Go to Files Screen and validate text contents", async () => {
    // Click on Files Button and assert contents on screen are correct
    await UplinkMainScreen.filesButton.click()
    await expect(await FilesScreen.filesTitle).toBeDisplayed()
    await expect(await FilesScreen.filesTitle).toHaveTextContaining("Files")
    await expect(await FilesScreen.folderName).toBeDisplayed()
    await expect(await FilesScreen.folderName).toHaveTextContaining("Folder 1")
    await expect(await FilesScreen.availableSpaceIndicatorText).toBeDisplayed()
    await expect(
      await FilesScreen.availableSpaceIndicatorText,
    ).toHaveTextContaining(/[. 0-9] Free/i)
    await expect(await FilesScreen.usedSpaceIndicatorText).toBeDisplayed()
    await expect(await FilesScreen.usedSpaceIndicatorText).toHaveTextContaining(
      [/[. 0-9]+(KB|MB|GB) /, "/ ", /. 0-9]+(KB|MB|GB)/i],
    )
  })

  it("Click on Folder 1 and validate that subfolders are displayed", async () => {
    // Click on main Folder from directory tree
    await (await FilesScreen.folderName).click()

    // Locate subfolder elements
    const firstSubfolder = await $(getPredicateForTextValueEqual("Subdir1"))
    const secondSubfolder = await $(getPredicateForTextValueEqual("Subdir2"))
    const thirdSubfolder = await $(getPredicateForTextValueEqual("f3"))

    // Assert subfolders are displayed and texts are matching
    await expect(firstSubfolder).toBeDisplayed()
    await expect(firstSubfolder).toHaveTextContaining("Subdir1")
    await expect(secondSubfolder).toBeDisplayed()
    await expect(secondSubfolder).toHaveTextContaining("Subdir2")
    await expect(thirdSubfolder).toBeDisplayed()
    await expect(thirdSubfolder).toHaveTextContaining("f3")
  })

  it("Files Directory Tree - Display the whole tree", async () => {
    // Locate subfolder elements and click on them
    await $(getPredicateForTextValueEqual("Subdir1")).click()
    await $(getPredicateForTextValueEqual("Subdir2")).click()
    await $(getPredicateForTextValueEqual("Subdir3")).click()

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(7)

    // Assert subfolders are displayed and texts are matching
    await expect(await $(getPredicateForTextValueEqual("f1"))).toBeDisplayed()
    await expect(await $(getPredicateForTextValueEqual("f2"))).toBeDisplayed()
    await expect(await $(getPredicateForTextValueEqual("f3"))).toBeDisplayed()
  })

  it("Files Directory Tree - Hide children elements", async () => {
    // Locate Subdir2 element and click on it
    await $(getPredicateForTextValueEqual("Subdir2")).click()

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(5)

    // Assert subfolders from Subdir2 does not exist in screen
    await expect(
      await $(getPredicateForTextValueEqual("Subdir3")),
    ).not.toExist()
    await expect(await $(getPredicateForTextValueEqual("f2"))).not.toExist()
  })

  it("Files Directory Tree - Hide all the tree", async () => {
    // Locate Main Folder element and click on it
    await $(getPredicateForTextValueEqual("Folder 1")).click()

    // Ensure that directory tree length is matching with the number of folders/subfolders displayed on screen
    await expect(FilesScreen.directoryTreeElements).toBeElementsArrayOfSize(1)

    // Assert subfolders from Subdir2 does not exist in screen
    await expect(
      await $(getPredicateForTextValueEqual("Subdir1")),
    ).not.toExist()
    await expect(await $(getPredicateForTextValueEqual("f1"))).not.toExist()
    await expect(
      await $(getPredicateForTextValueEqual("Subdir2")),
    ).not.toExist()
    await expect(await $(getPredicateForTextValueEqual("f2"))).not.toExist()
  })

  xit("Files Navigation - Go to Home when no folders are created", async () => {})

  xit("Files Navigation - Create a new folder", async () => {})

  xit("Files Navigation - Create a folder with same name than other existing in same location", async () => {})

  xit("Files Navigation - Rename a folder", async () => {})

  xit("Files Navigation - Navigate into a subfolder", async () => {})

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
