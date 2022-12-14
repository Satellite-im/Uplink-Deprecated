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
    await (await CreatePinScreen.pinInput).addValue("\n")
    await expect(await CreatePinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with less than 4 characters", async () => {
    await (await CreatePinScreen.pinInput).setValue("123" + "\n")
    await expect(await CreatePinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await CreatePinScreen.invalidPinMessage).toHaveTextContaining(
      "Your pin must be at least 4 characters",
    )
  })

  it("Attempt to use a PIN with more than 6 characters and assert error message", async () => {
    await (await CreatePinScreen.pinInput).setValue("1234567890").then(() => {
      expect(CreatePinScreen.maxLengthMessage).toBeDisplayed()
      expect(CreatePinScreen.maxLengthMessage).toHaveTextContaining(
        "Only four to six characters allowed",
      )
    })
  })

  it("Type a valid PIN with 4 characters and go to next page", async () => {
    await (await CreatePinScreen.pinInput).setValue("1234" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
    await driver.reset()
  })

  it("Type a valid PIN with 6 characters and go to next page", async () => {
    await (await CreatePinScreen.pinInput).setValue("123456" + "\n")
    await expect(await CreateAccountScreen.headerText).toBeDisplayed()
  })

  it("Assert Create Username screen texts", async () => {
    await expect(CreateAccountScreen.headerText).toHaveTextContaining(
      "Create Account",
    )
    await expect(CreateAccountScreen.subtitleText).toHaveTextContaining(
      "It's free and fast, just tell us what you'd like your username to be.",
    )
  })

  it("Attempt to provide an empty username", async () => {
    await (await CreateAccountScreen.userInput).addValue("\n")
    await expect(await CreateAccountScreen.errorMessageUsername).toBeDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining("Username is required")
  })

  it("Attempt to provide a username with less than 4 characters", async () => {
    await (await CreateAccountScreen.userInput).setValue("a" + "\n")
    await expect(await CreateAccountScreen.errorMessageUsername).toBeDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining(
      "Username needs to be between 4 and 32 characters long",
    )
  })

  it("Attempt to provide a username with less more than 32 characters", async () => {
    await (
      await CreateAccountScreen.userInput
    ).setValue("12345678901234567890123456789012345" + "\n") // Typing 35 characters
    await expect(await CreateAccountScreen.errorMessageUsername).toBeDisplayed()
    await expect(
      await CreateAccountScreen.errorMessageUsername,
    ).toHaveTextContaining(
      "Username needs to be between 4 and 32 characters long",
    )
  })

  it("Provide a valid username and go to next page", async () => {
    await (await CreateAccountScreen.userInput).setValue("qatest01" + "\n")
    await expect(await UplinkMainScreen.noActiveChatsText).toBeDisplayed()
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  xit("Reset app and assert Enter Pin Screen Texts", async () => {
    await driver.reset()
    await expect(EnterPinScreen.headerText).toHaveTextContaining("Enter Pin")
    await expect(EnterPinScreen.subtitleText).toHaveTextContaining(
      "Enter pin to unlock your account.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  xit("Enter an empty pin and assert error message", async () => {
    await (await EnterPinScreen.pinInput).addValue("\n")
    await expect(await EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  xit("Enter an wrong pin value and assert error message", async () => {
    await (await EnterPinScreen.pinInput).setValue("9999" + "\n")
    await expect(await EnterPinScreen.invalidPinMessage).toBeDisplayed()
    await expect(await EnterPinScreen.invalidPinMessage).toHaveTextContaining(
      "Invalid or incorrect pin supplied.",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  xit("Enter a PIN with more than 6 characters and assert error message", async () => {
    await (await EnterPinScreen.pinInput).setValue("1234567")
    await expect(await EnterPinScreen.maxLengthMessage).toBeDisplayed()
    await expect(await EnterPinScreen.maxLengthMessage).toHaveTextContaining(
      "Only four to six characters allowed",
    )
  })

  // Skipped for now since driver.reset() is redirecting to Create Pin Screen instead of Enter Pin Screen
  xit("Enter a valid PIN to be redirected to main screen", async () => {
    await (await EnterPinScreen.pinInput).setValue("123456" + "\n")
    await expect(await UplinkMainScreen.noActiveChatsText).toBeDisplayed()
  })
})
