main_ink_file="help.ink"

# Make sure there is no existing TEMmy build
if [ -d temmy ]; then rm -rf temmy; fi

# Compile the protocol Ink
if [[ $(uname) == *"MINGW"* ]]; then
    inklecate/inklecate.exe ${main_ink_file}
else
    mono inklecate/inklecate.exe ${main_ink_file}
fi


# Make the static site
cp -r boilerplate temmy
echo "var storyContent = $(cat ${main_ink_file}.json);" > temmy/story.js
cp "index.html" "temmy/"
cp -r "images" "temmy/"
cp "temmy.js" "temmy/"

# Clean up
rm ${main_ink_file}.json

# Try to copy to internal.connectomes.utah.edu/temmy
out_path=/y/Volumes/temmy
if [ -d $out_path ]; then rm -rf $out_path; fi
if [ -d /y/Volumes ]; then cp -r temmy $out_path; fi