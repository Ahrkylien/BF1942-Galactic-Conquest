subshader "tie_adv_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588 0.588 0.588;
	lightingSpecular false;
	envmap true;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced";
}
subshader "tie_adv_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 1 1 1;
	lightingSpecular false;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 97.0;
	envmap true;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced_win";
}
