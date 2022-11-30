global = Application
    .location = Location
    .unknown = Unknown
    .badge = Badge
    .account = Account
    .username = Username
    .status = Status
    .save = Save
    .edit = Edit
    .create = Create
    .message = Message

developer = Developer
    .view-source = View Source

account = Account
    .status = { global.status }
        .placeholder = Some { global.status } {global.message}...
        .save = { global.save } { global.status }
    .edit = { global.edit } {account}

prelude = Prelude
    .register = Register
        .choose_username = Choose { global.username }
        .create = {global.create} { global.account }

choose-username = Choose a { global.username }
