subshader "tie_adv_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced";
}

subshader "tie_adv_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.0 0.0 0.0;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 97.0;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/notexture";
}

