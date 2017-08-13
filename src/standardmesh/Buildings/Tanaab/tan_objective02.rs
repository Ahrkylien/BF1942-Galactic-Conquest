subshader "tie_adv_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.5 0.5 0.5;
	materialSpecular 0.2 0.2 0.2;
	materialSpecularPower 4.0;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced_prototype";
}

subshader "tie_adv_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.5 0.5 0.5;
	materialSpecular 0.2 0.2 0.2;
	materialSpecularPower 4.0;
	envmap true;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced_win";
}

