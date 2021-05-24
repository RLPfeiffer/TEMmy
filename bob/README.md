# bob the builder

Makes our dataset building easier by running the scripts automatically when it's safe to do so.
Pipes script output through a processor that prunes common useless warnings and highlights
important errors and other information.

Setup:

1. Install [rito](github.com/nqnstudios/rito) and set up environment variables for rito slack support
1. Map network drives for RawData, W:, DROPBOX, Notification, etc. 
1. copy RC2-config.yaml or Build1-config.yaml to your Python Scripts directory/bob-config.yaml and edit the values as needed