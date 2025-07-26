function Copy-Assets {
	[CmdletBinding()]
	param (
		[string]$SourceDir,
		[string]$TargetDir
	)
	Write-Host "Source dir $SourceDir"
	Write-Host "Target dir $TargetDir"
	$items = Get-ChildItem -Path "$SourceDir*" -Include *.png, *.ttf, *.ron, *.csv, *.glb -Exclude *uv.png, *.kra, *~
	ForEach ($i in $items) {
		Write-Host "Copying $i"
		Copy-Item $i $TargetDir -Force
	}
}

Write-Host "Copying images"
Copy-Assets "./assets_wip/images/" "./assets/images/"
Copy-Assets "./assets_wip/images/logo/" "./assets/images/logo/"

Write-Host "Copying fonts"
Copy-Assets "./assets_wip/fonts/" "./assets/fonts/"

# Write-Host "Copying map data"
# Copy-Assets "./assets_wip/map/" "./assets/map/"
# Copy-Assets "./assets_wip/map/tiles/" "./assets/map/tiles/"
# Copy-Assets "./assets_wip/map/tiles/wall/" "./assets/map/tiles/wall/"
# Copy-Assets "./assets_wip/map/tiles/ground/" "./assets/map/tiles/ground/"
# Copy-Assets "./assets_wip/map/tiles/tree/" "./assets/map/tiles/tree/"

# Write-Host "Copying level data"
# Copy-Assets "./assets_wip/levels/" "./assets/levels/"

# Copy-Assets "./assets_wip/buildings/" "./assets/buildings/"
# Copy-Assets "./assets_wip/buildings/hq/" "./assets/buildings/hq/"
# Copy-Assets "./assets_wip/buildings/mine/" "./assets/buildings/mine/"
# Copy-Assets "./assets_wip/buildings/farm/" "./assets/buildings/farm/"
# Copy-Assets "./assets_wip/buildings/grain_silo/" "./assets/buildings/grain_silo/"
# Copy-Assets "./assets_wip/buildings/magazine/" "./assets/buildings/magazine/"
# Copy-Assets "./assets_wip/buildings/advanced_farm/" "./assets/buildings/advanced_farm/"
# Copy-Assets "./assets_wip/buildings/steam_mine/" "./assets/buildings/steam_mine/"
# Copy-Assets "./assets_wip/buildings/barracks/" "./assets/buildings/barracks/"
# Copy-Assets "./assets_wip/buildings/farrier/" "./assets/buildings/farrier/"
# Copy-Assets "./assets_wip/buildings/foundry/" "./assets/buildings/foundry/"
# Copy-Assets "./assets_wip/buildings/horologist/" "./assets/buildings/horologist/"
# Copy-Assets "./assets_wip/buildings/sprocketmaster/" "./assets/buildings/sprocketmaster/"
# Copy-Assets "./assets_wip/buildings/artificer/" "./assets/buildings/artificer/"
# Copy-Assets "./assets_wip/buildings/steam_laboratory/" "./assets/buildings/steam_laboratory/"


# Copy-Assets "./assets_wip/units/" "./assets/units/"
# Copy-Assets "./assets_wip/units/worker/" "./assets/units/worker/"
# Copy-Assets "./assets_wip/units/linesman/" "./assets/units/linesman/"

# Write-Host "Copying maps"
# Copy-Assets "./assets_wip/maps/" "./assets/maps/"
# Copy-Assets "./assets_wip/maps/story/0_sovereign_landing/" "./assets/maps/story/0_sovereign_landing/"

# Write-Host "Copying maps ron"
# Copy-Assets "./assets_wip/maps/story/" "./assets/maps/story/"
# Copy-Assets "./assets_wip/maps/conquest/" "./assets/maps/conquest/"
# Copy-Assets "./assets_wip/maps/skirmish/" "./assets/maps/skirmish/"

# Write-Host "Copying tile assets"
# Copy-Assets "./assets_wip/map/tile/" "./assets/map/tile/"

# Write-Host "Copying room assets"
# Copy-Assets "./assets_wip/map/tile/rooms/" "./assets/map/tile/rooms/"

# Write-Host "Copying building assets"
# Copy-Assets "./assets_wip/map/tile/building/" "./assets/map/tile/building/"

# Write-Host "Copying campaign assets"
# Copy-Assets "./assets_wip/campaign/" "./assets/campaign/"