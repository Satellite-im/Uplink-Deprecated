import config from "./wdio.shared.local.appium.conf"

// ============
// Specs
// ============
config.specs = ["./tests/specs/**/*.spec.ts"]

// ============
// Capabilities
// ============
// For all capabilities please check
// http://appium.io/docs/en/writing-running-appium/caps/#general-capabilities
config.capabilities = [
  {
    // The defaults you need to have in your config
    platformName: "mac",
    maxInstances: 1,
    // For W3C the appium capabilities need to have an extension prefix
    // http://appium.io/docs/en/writing-running-appium/caps/
    // This is `appium:` for all Appium Capabilities which can be found here
    "appium:automationName": "mac2",
    "appium:prerun": {
      command: 'do shell script "rm -rf ~/.warp"',
    },
    // @ts-ignore
    "appium:bundleId": "im.satellite.uplink",
    "appium:newCommandTimeout": 15,
  },
]

exports.config = config
