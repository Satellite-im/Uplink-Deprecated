import PinDesktopScreen from "../screenobjects/PinDesktopScreen"
import CreateAccountScreen from "../screenobjects/CreateAccountScreen"

describe("Pin Screen Desktop", async () => {
  beforeEach(async () => {
    await PinDesktopScreen.waitForIsShown(true)
  })

  it("Assert screen texts", async () => {
    await expect(PinDesktopScreen.headerText).toHaveTextContaining(
      "Create a Pin",
    )
    await expect(PinDesktopScreen.subtitleText).toHaveTextContaining(
      "Choose a 4-6 digit pin to secure your account.",
    )
  })

  it("Attempt to use an empty PIN", async () => {
    await (await PinDesktopScreen.pinInput).addValue("\n")
    await expect(await PinDesktopScreen.invalidPinMessage).toBeDisplayed()
    await expect(await PinDesktopScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with less than 4 characters", async () => {
    await (await PinDesktopScreen.pinInput).setValue("123" + "\n")
    await expect(await PinDesktopScreen.invalidPinMessage).toBeDisplayed()
    await expect(await PinDesktopScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Type a valid PIN with 4 characters and go to next page", async () => {
    await (await PinDesktopScreen.pinInput).setValue("1234" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
    await driver.reset()
  })

  it("Type a valid PIN with 6 characters and go to next page", async () => {
    await (await PinDesktopScreen.pinInput).setValue("123456" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
  })
})
