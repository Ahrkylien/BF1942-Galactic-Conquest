subshader "yt600_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	twosided true;
	texture "texture/vehicles/yt600/repaircraft_id1";
}

subshader "yt600_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	twosided true;
	texture "texture/vehicles/yt600/repaircraft_id2";
}

subshader "yt600_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 0.584314 0.584314 0.584314;
	materialSpecular 0.8 0.8 1.0;
	materialSpecularPower 2.0;
	transparent true;
	twosided true;
	envmap true;
	texture "texture/vehicles/yt600/repaircraft_id1";
}

