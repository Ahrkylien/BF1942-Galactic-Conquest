subshader "material_0" "StandardMesh/Default"
{
	texture "texture/Vehicles/Eta-2/Eta-2-red";
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
}

subshader "material_1" "StandardMesh/Default"
{
	texture "texture/Vehicles/Eta-2/glass";
	transparent true;
	lightingSpecular true;
	twosided true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	materialSpecular 1.0 1.0 1.0;
	materialSpecularPower 12.5;
}

