# Uplink - UI Test Automation Framework

Test Automation Framework designed to create UI tests in webdriverIO using Appium for MacOS Uplink application now, and Windows app in the future

## Based on

This automation framework is currently based on:

- **WebdriverIO:** `7.19.7`
- **Appium:** `2.0.0`

## Setting up to run on the local machine

1. First, install all the required dependencies
```sh
cd appium-tests && npm install
```
2. Install Appium on a local machine. You can find detailed instructions for this process [here](https://appium.io/docs/en/about-appium/getting-started/)
3. Add appium drivers required to execute the tests on the desired platform. For now, the framework only works for macOS
```sh
# To Install Appium Mac2 Driver to run the tests on macOS
appium driver install mac2
```
4. Though this should not be required to run MacOS or Windows tests, you can run appium-doctor to validate that all Appium necessary dependencies were installed correctly [here](https://github.com/appium/appium-doctor). If Android/iOS environment variables are required, these can be added by editing the local ~/.zshrc file:
```sh
# Open .zshrc file
nano ~/.zshrc

# Add the following environment variables to your .zshrc file:
export PATH="/Users/yourusername/Library/Android/sdk/tools:$PATH"
export PATH="/Users/yourusername/Library/Android/sdk/platform-tools:$PATH"
export JAVA_HOME="/Library/Java/JavaVirtualMachines/jdk-18.0.1.1.jdk/Contents/Home" # Replace with the JDK folder from the JDK version installed on your machine
export ANDROID_HOME="/Users/yourusername/Library/Android/sdk"
```

5. Ask the development team to provide the latest .dmg file for macOS testing or the .exe file for Windows testing. Then, install the application manually on your local machine
6. Once the application is installed, you can run the tests by using the following commands:
```sh
# To run the tests under MacOS
npm run mac.app
```

## Configuration files

This framework uses a specific config for macOS now, and will contain configuration for Windows in the future, see [configs](./config). The configs are based on a shared config
[`wdio.shared.conf.ts`](./config/wdio.shared.conf.ts).
This shared config holds **all the defaults** so the macOS and Windows configs only need to hold the capabilities and specs that are needed
for running on macOS and/or Windows.

Please check the [`wdio.shared.conf.ts`](./config/wdio.shared.conf.ts)-file for the minimal configuration options. Notes are added for why
a different value has been selected in comparison to the default values WebdriverIO provides.

Since we do not have Appium installed as part of this package we are going to use the globally installed version of Appium. This is
configured in [`wdio.shared.local.appium.conf.ts`](./config/wdio.shared.local.appium.conf.ts).

Finally, since there will be a GitHub action setup to run the appium tests on macOS, there will be one configuration file used to run these tests on CI. This will be configured in [`wdio.macos.ci.conf.ts`](./config/wdio.macos.ci.conf.ts).

## Locator strategy for native apps

The locator strategy for this Test Automation Framework is to preferably use `accessibilityID`'s. `AccessibilityID`'s make it easy to script once and run on macOS and Windows because most of the apps already have some `accessibilityID`'s.

If `accessibilityID`'s can't be used, for example, then for Mac2 driver, -ios class chain or -ios predicate string should be preferred as locators. Finally, the last option to use could be XPATH, which is not preferred because these can be changed without notice for us when new UI elements are added to the screens.

## Improvements to be implemented soon

- CI job to run the tests on every PR under GitHub Actions - To work on this, we need to have a properly codesigned application
- For now, to run the tests, we need to manually install the application on our local machine, before running the tests
- Tests running on Windows - To add these, we need to start adding the windows UI locators for the elements and then modify the tests to run on both platforms
- Tests running on Ubuntu - To add these, we need to start adding the Ubuntu UI locators for the elements and then modify the tests to run on both platforms. Also, there is no official driver for appium to run tests under Ubuntu. Unfortunately, there is only one third-party driver that we need to validate that it is secure and works correctly before implementing it inside the project
