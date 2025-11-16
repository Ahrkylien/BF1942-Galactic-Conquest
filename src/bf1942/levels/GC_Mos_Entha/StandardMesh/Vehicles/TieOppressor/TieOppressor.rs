subshader "material_0" "StandardMesh/Default"
{
	texture "texture/vehicles/TieOppressor/tie_oppressor";
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
}

subshader "material_1" "StandardMesh/Default"
{
	texture "texture/vehicles/TieOppressor/tie_oppressor_glass";
	transparent true;
	twosided true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
}

subshader "material_2" "StandardMesh/Default"
{
	texture "texture/vehicles/TieOppressor/tie_oppressor_2";
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
}

