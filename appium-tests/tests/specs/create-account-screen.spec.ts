import CreatePinScreen from "../screenobjects/CreatePinScreen"
import CreateAccountScreen from "../screenobjects/CreateAccountScreen"
import EnterPinScreen from "../screenobjects/EnterPinScreen"
import UplinkMainScreen from "../screenobjects/UplinkMainScreen"

describe("Create Account on Uplink Desktop", async () => {
  before(async () => {
    await CreatePinScreen.waitForIsShown(true)
  })

  it("Assert Create PIN screen texts", async () => {
    await expect(CreatePinScreen.headerText).toHaveTextContaining(
      "Create a Pin",
    )
    await expect(CreatePinScreen.subtitleText).toHaveTextContaining(
      "Choose a 4-6 digit pin to secure your account.",
    )
  })

  it("Attempt to use an empty PIN", async () => {
    await CreatePinScreen.enterPin("\n")
    await CreatePinScreen.invalidPinMessage.waitForDisplayed()
    await expect(CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with less than 4 characters", async () => {
    await CreatePinScreen.enterPin("123" + "\n")
    await CreatePinScreen.invalidPinMessage.waitForDisplayed()
    await expect(CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with more than 6 characters and assert error message", async () => {
    await CreatePinScreen.enterPin("1234567890")
    await CreatePinScreen.maxLengthMessage.waitForDisplayed()
    await expect(CreatePinScreen.maxLengthMessage).toHaveTextContaining(
      "Only four to six characters allowed",
    )
  })

  it("Type a valid PIN with 4 characters and go to next page", async () => {
    await CreatePinScreen.enterPin("1234" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
  })

  it("Type a valid PIN with 6 characters and go to next page", async () => {
    await CreatePinScreen.resetApp()
    await CreatePinScreen.enterPin("123456" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
  })

  it("Assert Create Username screen texts", async () => {
    await expect(await CreateAccountScreen.headerText).toHaveTextContaining(
      "Create Account",
    )
    await expect(CreateAccountScreen.subtitleText).toHaveTextContaining(
      "It's free and fast, just tell us what you'd like your username to be.",
    )
  })

  it("Attempt to provide an empty username", async () => {
    await CreateAccountScreen.enterUsername("")
    await CreateAccountScreen.errorMessageUsername.waitForDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining("Username is required")
  })

  it("Attempt to provide a username with less than 4 characters", async () => {
    await CreateAccountScreen.enterUsername("a")
    await CreateAccountScreen.errorMessageUsername.waitForDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining(
      "Username needs to be between 4 and 32 characters long",
    )
  })

  it("Attempt to provide a username with less more than 32 characters", async () => {
    // Typing 35 characters
    await CreateAccountScreen.enterUsername(
      "12345678901234567890123456789012345",
    )
    await CreateAccountScreen.errorMessageUsername.waitForDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining(
      "Username needs to be between 4 and 32 characters long",
    )
  })

  it("Provide a valid username and go to next page", async () => {
    await CreateAccountScreen.enterUsername("qatest01")
    await expect(UplinkMainScreen.noActiveChatsText).toBeDisplayed()
  })
})
