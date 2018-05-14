##How to run

- Run 10.linux.android.lib.build.sh

- The generated artifacts will be in build/outputs/aar/vcx-*.aar

- Copy vcx-debug.aar and past it in vcx-debug folder in Wrapper test.


##Steps to include aar in new project

- In Android project Add new Module > Import Jar/AAR

- Select the aar file you want to include

- A new module will be created (Similar to vcx-debug folder in WrapperTest sample)

- In app/build.gradle file add `compile project(':<YOUR_PROJECT_NAME>')`