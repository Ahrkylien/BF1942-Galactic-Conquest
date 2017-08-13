subshader "lancer_interior_Material0" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/lancer/diamond_plate";
}

subshader "lancer_interior_Material1" "StandardMesh/Default"
{
	lighting false;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/lancer/groundlight";
}

subshader "lancer_interior_Material2" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 0.1 0.1 0.1;
	texture "texture/vehicles/lancer/floor";
}

subshader "lancer_interior_Material3" "StandardMesh/Default"
{
	lighting false;
	envmap true;
	materialDiffuse 0.1 0.1 0.1;
	texture "texture/vehicles/lancer/rooflight";
}

subshader "lancer_interior_Material4" "StandardMesh/Default"
{
	lighting true;
	envmap false;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	materialDiffuse 0.5 0.5 0.5;
	texture "texture/vehicles/lancer/wall";
}

subshader "lancer_interior_Material5" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	lightingSpecular true;
	materialSpecular 0.8 0.8 0.8;
	materialSpecularPower 4.0;
	materialDiffuse 0.2 0.2 0.2;
	texture "texture/vehicles/lancer/contpan";
}

subshader "lancer_interior_Material6" "StandardMesh/Default"
{
	lighting true;
	envmap true;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/lancer/tpossition";
}

