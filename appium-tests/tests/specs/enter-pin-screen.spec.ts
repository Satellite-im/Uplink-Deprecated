import EnterPinScreen from "../screenobjects/EnterPinScreen"
import UplinkMainScreen from "../screenobjects/UplinkMainScreen"

describe("Create Account on Uplink Desktop", async () => {
  before(async () => {
    await EnterPinScreen.waitForIsShown(true)
  })

  it("Assert Enter Pin Screen Texts", async () => {
    await expect(EnterPinScreen.headerText).toHaveTextContaining("Enter Pin")
    await expect(EnterPinScreen.subtitleText).toHaveTextContaining(
      "Enter pin to unlock your account.",
    )
  })

  it("Enter an empty pin and assert error message", async () => {
    await EnterPinScreen.enterPin("\n")
    await expect(EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  it("Enter an wrong pin value and assert error message", async () => {
    await EnterPinScreen.enterPin("9999" + "\n")
    await expect(EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  it("Enter a PIN with more than 6 characters and assert error message", async () => {
    await EnterPinScreen.enterPin("98765432")
    await expect(EnterPinScreen.maxLengthMessage).toBeDisplayed()
    await expect(EnterPinScreen.maxLengthMessage).toHaveTextContaining(
      "Only four to six characters allowed",
    )
  })

  it("Enter a valid PIN to be redirected to main screen", async () => {
    await EnterPinScreen.enterPin("123456" + "\n")
    await expect(UplinkMainScreen.noActiveChatsText).toBeDisplayed()
  })
})
