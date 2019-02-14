# Audio sources.
# All audio from archive.org

DL=wget -O $@

all: tracks/things.mp3 tracks/squatty.mp3 tracks/rabbits.mp3 tracks/campus.mp3 tracks/bess.mp3 tracks/rent.mp3 tracks/stars.mp3 tracks/blues.mp3 tracks/things.flac tracks/squatty.flac tracks/rabbits.flac tracks/campus.flac tracks/bess.flac tracks/rent.flac tracks/stars.flac tracks/blues.flac

tracks:
	mkdir -p tracks

tracks/things.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraThingsAintWhatTheyUsedToBe1941/Johnny_Hodges_Orchestra-Things_Aint_What_They_Used_To_Be-1941.mp3

tracks/squatty.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraSquatyRoo1941/Johnny_Hodges_Orchestra-Squaty_Roo-1941.mp3

tracks/rabbits.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-TheRabbitsJump/Johnny_Hodges_Orchestra-The_Rabbits_Jump-1939.mp3

tracks/campus.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-SwinginOnTheCampus/Johnny_Hodges_Orchestra-Swingin_On_The_Campus-1939.mp3

tracks/bess.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-GoodQueenBess/Johnny_Hodges_Orchestra-Good_Queen_Bess-1940.mp3

tracks/rent.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraRentPartyBlues1939/Johnny_Hodges_Orchestra-Rent_Party_Blues-1939.mp3

tracks/stars.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-DancingOnTheStars/Johnny_Hodges_Orchestra-Dancing_On_The_Stars-1938.mp3

tracks/blues.mp3: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-ThatsTheBluesOldMan/Johnny_Hodges_Orchestra-Thats_The_Blues_Old_Man-1940.mp3

tracks/things.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraThingsAintWhatTheyUsedToBe1941/Johnny_Hodges_Orchestra-Things_Aint_What_They_Used_To_Be-1941.flac

tracks/squatty.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraSquatyRoo1941/Johnny_Hodges_Orchestra-Squaty_Roo-1941.flac

tracks/rabbits.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-TheRabbitsJump/Johnny_Hodges_Orchestra-The_Rabbits_Jump-1939.flac

tracks/campus.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-SwinginOnTheCampus/Johnny_Hodges_Orchestra-Swingin_On_The_Campus-1939.flac

tracks/bess.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-GoodQueenBess/Johnny_Hodges_Orchestra-Good_Queen_Bess-1940.flac

tracks/rent.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestraRentPartyBlues1939/Johnny_Hodges_Orchestra-Rent_Party_Blues-1939.flac

tracks/stars.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-DancingOnTheStars/Johnny_Hodges_Orchestra-Dancing_On_The_Stars-1938.flac

tracks/blues.flac: tracks
	$(DL) https://archive.org/download/JohnnyHodgesOrchestra-ThatsTheBluesOldMan/Johnny_Hodges_Orchestra-Thats_The_Blues_Old_Man-1940.flac

clean:
	rm -rf tracks