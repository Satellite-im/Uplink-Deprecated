import FilesScreen from "../screenobjects/FilesScreen"
import CreatePinScreen from "../screenobjects/CreatePinScreen"
import CreateAccountScreen from "../screenobjects/CreateAccountScreen"
import UplinkMainScreen from "../screenobjects/UplinkMainScreen"
import { customPredicateString } from "../helpers/commands"

describe("Files Screen Tests on Uplink Desktop", async () => {
  before(async () => {
    await CreatePinScreen.waitForIsShown(true)
    await (await CreatePinScreen.pinInput).setValue("1234" + "\n")
    await CreateAccountScreen.waitForIsShown(true)
    await (await CreateAccountScreen.userInput).setValue("filestest01" + "\n")
    await UplinkMainScreen.waitForIsShown(true)
  })

  it("Go to Files Screen and validate text contents", async () => {
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
    await (await FilesScreen.folderName).click()
    const firstSubfolder = await $(
      customPredicateString("48", "value", "Subdir1"),
    )
    const secondSubfolder = await $(
      customPredicateString("48", "value", "Subdir2"),
    )
    const thirdSubfolder = await $(customPredicateString("48", "value", "f3"))
    await expect(firstSubfolder).toBeDisplayed()
    await expect(firstSubfolder).toHaveTextContaining("Subdir1")
    await expect(secondSubfolder).toBeDisplayed()
    await expect(secondSubfolder).toHaveTextContaining("Subdir2")
    await expect(thirdSubfolder).toBeDisplayed()
    await expect(thirdSubfolder).toHaveTextContaining("f3")
  })
})
