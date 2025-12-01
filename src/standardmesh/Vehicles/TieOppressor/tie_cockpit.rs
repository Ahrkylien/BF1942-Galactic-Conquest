subshader "tie_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.25 0.25 0.25;
	lightingSpecular true;
	materialSpecular 0.1 0.1 0.1;
	materialSpecularPower 1.0;
	envmap true;
	texture "texture/Vehicles/tiefighter/Window";
}

subshader "tie_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.25 0.25 0.25;
	lightingSpecular true;
	materialSpecular 0.1 0.1 0.1;
	materialSpecularPower 1.0;
	envmap true;
	texture "texture/Vehicles/tiefighter/Cockpit";
}

subshader "tie_cockpit_Material2" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.25 0.25 0.25;
	lightingSpecular true;
	materialSpecular 0.1 0.1 0.1;
	materialSpecularPower 1.0;
	envmap true;
	texture "texture/Vehicles/tiefighter/Hud_Screens";
}

subshader "tie_cockpit_Material3" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.25 0.25 0.25;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
        transparent true;
        depthWrite false;
	envmap false;
	texture "texture/Vehicles/tiefighter/Windsheild";
}

subshader "tie_cockpit_Material4" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.25 0.25 0.25;
	lightingSpecular true;
	materialSpecular 0.1 0.1 0.1;
	materialSpecularPower 1.0;
	envmap true;
	texture "Mods\GCMOD\Movies\tietest.bik";
}

