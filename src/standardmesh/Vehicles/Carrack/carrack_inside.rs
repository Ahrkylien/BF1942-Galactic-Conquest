subshader "carrack_inside_Material0" "StandardMesh/Default"
{
	lighting true;
	envmap false;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	materialDiffuse 0.1 0.1 0.1;
	texture "texture/vehicles/carrack/wall";
}

subshader "carrack_inside_Material1" "StandardMesh/Default"
{
	lighting false;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/carrack/roof";
}

subshader "carrack_inside_Material2" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/carrack/floor";
}

subshader "carrack_inside_Material3" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/carrack/body5";
}

subshader "carrack_inside_Material4" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/carrack/body3";
}

subshader "carrack_inside_Material5" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/carrack/floor2";
}

subshader "carrack_inside_Material6" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/buildings/judicator/jud_frontpanel";
}

