For /F "Tokens=*" %%I in ('curl "https://chromedriver.storage.googleapis.com/LATEST_RELEASE"') Do Set version=%%I
curl -Lo chromedriver_win32.zip "https://chromedriver.storage.googleapis.com/%version%/chromedriver_win32.zip"
powershell -command "Expand-Archive -Force .\chromedriver_win32.zip '%~dp0'"
del chromedriver_win32.zip