# # WarpGUI Testing
The following is a step-by-step list you should follow to ensure functionality when physically or automatically testing WarpGUI.

Tests marked with `[NYI] Not Yet Implemented` do not need to pass yet. 

## Compilation
1. `cargo run` should not return any errors.
2. `cargo run` should not return any warnings.

## Onboarding
1. Create Pin
	1.  Should be able to enter any alpha numeric key to input a new character into the pin.
	2. Should not be able to use modifier keys and spacebar to enter pin.
	3. Should be able to backspace characters from the pin.
		1.  Should delete a character from the pin when pressing the `backspace / delete` key.
		2. Should do nothing when pressing the `backspace / delete` key when the pin is empty.
	4. Should be able to enter up to 6 characters.
	5. Should not be able to submit with less than 4 characters.
	6. Should be able to leave the app then click back into the app and resume all above tests.
	7. Should submit when clicking “check” button.
	8. Should submit when pressing the `enter` key.
2. Create Account
	1. Should be able to enter a username.
	2. Should be able to click around and use keyboard shortcuts (such as copy paste) to create the name.
	3. Submit button should be clickable once username is valid.
	4. Submit button should not be clickable when username is invalid.
	5. **NYI ** Should be able to upload an image for the profile picture. 
	6. Clicking the create account button should take you to the main application screen.
	
## Unlock
1. Should display the following when entering a pin
	1. Error text stating the pin didn’t work.
	2. Pin should turn red.
	3. “Check” button should not display.
	4. **NYI ** Pin should shake.
2. Should be able to delete an incorrect pin to try again.
3. Should take you to the main page when entering the correct pin.
4. Should not take you to any other page.

## Friends
1. Should display a button in the sidebar if you have no friends yet.
	1. Should open friend modal when clicked.
2. Should open friends modal when clicking the `users` icon in the menu bar.
3. Should copy `DIDKey` to clipboard when clicking the “Copy Code” button.
4. Should be able to interact with the add friend input.
	1. Should be able to paste a `DIDKey`.
	2. Should not be able to add yourself.
	3. Should be able to send request to valid `DIDKey`.
	4. Should return error when invalid `DIDKey` is supplied.
	5. Should submit when pressing the `enter` key.
	6. Should submit when pressing the “add” button.
5. Should show friend request on remote instance of the app.
6. Should allow remote to accept the request.
7. Should allow remote to deny request.
	1. Should remove outgoing request from origin’s account.
8. Should show outgoing request on origin instance of the app.
	1. Should remove outgoing request on origin if remote denies request.
9. Should show list of active friends.
10. Should allow clicking the “chat” button to start a chat with friend.

## Compose
1. Should be able to focus and type a multi-line message in the compose input.
2. Should be able to resize when multiple lines of text exist inside the input.
3. Should be able to use keyboard shortcuts to select, copy, paste and otherwise modify the message.
4. Should be able to send message by hitting either return or the send button.
5. Should clear the input when message is sent.
6. Should show placeholder text when the input is empty.

