subshader "tie_adv_Material0" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.588 0.588 0.588;
	lightingSpecular false;
	twosided true;
	texture "texture/Vehicles/TieAdvanced/tie_advanced";
}
subshader "tie_adv_Material1" "StandardMesh/Default"
{
	lighting true;
	materialDiffuse 0.0 0.0 0.0;
	lightingSpecular true;
	materialSpecular 0.9 0.9 0.9;
	materialSpecularPower 97.0;
	twosided true;
	texture "texture/buildings/superecho/part_2_c_roof";
}
