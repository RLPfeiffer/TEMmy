# Make sure there is no existing TEMmy build
if [ -d temmy ]; then rm -rf temmy; fi

# Compile the protocol Ink
if [[ $(uname) == *"MINGW"* ]]; then
    inklecate/inklecate.exe protocol.ink
else
    mono inklecate/inklecate.exe protocol.ink
fi


# Make the static site
cp -r boilerplate temmy
echo "var storyContent = $(cat protocol.ink.json);" > temmy/story.js
cp "index.html" "temmy/"
cp -r "images" "temmy/"
cp "temmy.js" "temmy/"

# Clean up
rm protocol.ink.json

# Try to copy to internal.connectomes.utah.edu/temmy
out_path=/y/Volumes/temmy
if [ -d $out_path ]; then rm -rf $out_path; fi
cp -r temmy $out_path