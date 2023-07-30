# LateralMovementDetection
Collecting, preparing and merging the Los Alamos National's Laboratory unified host data set and and redteam compromise events dataset to be used in a deep learning algorithm for lateral movement detection.

For details in implementation, please see [Bai et al., 2020](https://www.sciencedirect.com/science/article/abs/pii/S0140366420319617#:~:text=Lateral%20Movement%20(LM)%20is%20one,both%20host%20and%20network%20logs)

In short, the data consists of the "redteam data", consisting of the authentication logs generated from red team activities (e.g. compromise events). However, the redteam data is missing crucial information such as logoff events. To mitigate this, the authors suggest merging the unified data set, which consists of comprehensive Windows event logs, including logoff events. Further pre-processing is necessary: only a subset of EventIDs are relevant to the project, and only complete data consisting of remote login and source host should be included.

While the redteam dataset is fairly small, the unified dataset is much larger (approximately 35gb of compressed bzip2 files), resulting in a challenge for processing in regular use computers with limited RAM and HD. 

Here, I share an open-source bash script that solves that problem by downloading the bz2 files, decompressing it line by line to save RAM, checking if each data meets the selection criteria, and appending that data to a text file only if it does. As such, the processing can be done efficiently without risk of RAM limitations, and is only constrained by download speed.

The script does the following:

1. Download the redteam dataset
2. Download the unified dataset files, one by one
3. Once a unified dataset is downloaded, a Python script decompresses the enclosed .json file line by line
4. The script determines if the data matches the selection criteria, and appends to a text file if so
5. Once processing is finished, the unified dataset file is deleted to save HD memory.
