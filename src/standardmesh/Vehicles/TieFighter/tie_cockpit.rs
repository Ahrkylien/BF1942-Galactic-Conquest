subshader "tie_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	texture "texture/Vehicles/tiefighter/WINDOW";
}

subshader "tie_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	texture "texture/Vehicles/tiefighter/COCKPIT";
}

subshader "tie_cockpit_Material2" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/Vehicles/tiefighter/Hud_Screens";
}

subshader "tie_cockpit_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
        transparent true;
        depthWrite false;
	texture "texture/Vehicles/tiefighter/Windsheild";
}

