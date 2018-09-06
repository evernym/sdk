# Development
FROM libindy
ARG uid=1000
RUN useradd -ms /bin/bash -u $uid java
RUN usermod -aG sudo java


# Install Android SDK and NDK 
RUN mkdir -m 777 /home/java/android-sdk-linux
RUN wget https://dl.google.com/android/repository/tools_r25.2.3-linux.zip -P /home/java/android-sdk-linux
RUN unzip /home/java/android-sdk-linux/tools_r25.2.3-linux.zip -d /home/java/android-sdk-linux
RUN ls -al /home/java/android-sdk-linux
RUN yes | .//home/java/android-sdk-linux/tools/android update sdk --no-ui
RUN yes | .//home/java/android-sdk-linux/tools/bin/sdkmanager "ndk-bundle"

RUN echo "java ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers 

COPY vcx/libvcx/target/debian/*.deb .
RUN dpkg -i *.deb
USER java